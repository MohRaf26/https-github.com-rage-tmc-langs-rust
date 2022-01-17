//! Contains functions for zipping and unzipping projects.

pub use zip::result::ZipError;

use std::collections::HashSet;
use std::io::{Cursor, Read, Seek, Write};
use std::path::Path;
use std::path::PathBuf;
use tmc_langs_framework::error::TmcError;
use tmc_langs_framework::file_util;
use tmc_langs_framework::policy::StudentFilePolicy;
use walkdir::{DirEntry, WalkDir};
use zip::{write::FileOptions, ZipArchive, ZipWriter};

/// Zips the given directory, only including student files according to the given policy.
pub fn zip<P: StudentFilePolicy>(policy: P, root_directory: &Path) -> Result<Vec<u8>, TmcError> {
    let mut writer = ZipWriter::new(Cursor::new(vec![]));

    for entry in WalkDir::new(root_directory)
        .into_iter()
        .filter_entry(|e| !contains_tmcnosubmit(e))
        .filter_map(|e| e.ok())
    {
        log::trace!("processing {}", entry.path().display());
        if policy.is_student_file(entry.path(), &root_directory)? {
            let path = root_directory
                .parent()
                .map(|p| entry.path().strip_prefix(p).unwrap())
                .unwrap_or_else(|| entry.path());
            if entry.path().is_dir() {
                log::trace!("adding directory {}", path.display());
                writer
                    .add_directory(path_to_zip_compatible_string(path), FileOptions::default())?;
            } else {
                let bytes = file_util::read_file(entry.path())?;
                log::trace!("writing file {}", path.display());
                writer.start_file(path_to_zip_compatible_string(path), FileOptions::default())?;
                writer
                    .write_all(&bytes)
                    .map_err(|e| TmcError::ZipWrite(path.to_path_buf(), e))?;
            }
        }
    }
    let cursor = writer.finish()?;
    Ok(cursor.into_inner())
}

// ensures the / separator is used
fn path_to_zip_compatible_string(path: &Path) -> String {
    let mut string = String::new();
    for component in path.components() {
        if !string.is_empty() {
            string.push('/');
        }
        string.push_str(&*component.as_os_str().to_string_lossy());
    }
    string
}

// todo: remove
/// Finds a project directory in the given zip and unzips it according to the given student policy. Also cleans unnecessary non-student files.
///
/// First a project directory is found within the directory. Only files within the project directory are unzipped.
///
pub fn unzip<P>(
    policy: P,
    zip: impl std::io::Read + std::io::Seek,
    target: &Path,
) -> Result<(), TmcError>
where
    P: StudentFilePolicy,
{
    log::debug!("Unzipping to {}", target.display());

    let mut zip_archive = ZipArchive::new(zip)?;

    let project_dir = find_project_dir(&mut zip_archive)?;
    log::debug!("Project dir in zip: {}", project_dir.display());

    // for each file in the zip, contains its path if unzipped
    // used to clean non-student files not in the zip later
    let mut unzip_paths = HashSet::new();

    for i in 0..zip_archive.len() {
        let mut file = zip_archive.by_index(i)?;
        let file_path = PathBuf::from(file.name());
        let relative = match file_path.strip_prefix(&project_dir) {
            Ok(relative) => relative,
            _ => {
                log::trace!("skip {}, not in project dir", file.name());
                continue;
            }
        };
        let path_in_target = target.join(&relative);
        log::trace!("processing {:?} -> {:?}", file_path, path_in_target);

        if file.is_dir() {
            log::trace!("creating {:?}", path_in_target);
            file_util::create_dir_all(&path_in_target)?;
            unzip_paths.insert(
                path_in_target
                    .canonicalize()
                    .map_err(|e| TmcError::Canonicalize(path_in_target.clone(), e))?,
            );
        } else {
            let mut write = true;
            let mut file_contents = vec![];
            file.read_to_end(&mut file_contents)
                .map_err(|e| TmcError::ZipRead(file_path.clone(), e))?;
            // always overwrite .tmcproject.yml
            if path_in_target.exists()
                && !path_in_target
                    .file_name()
                    .map(|o| o == ".tmcproject.yml")
                    .unwrap_or_default()
            {
                let target_file_contents = file_util::read_file(&path_in_target)?;
                if file_contents == target_file_contents
                    || (policy.is_student_file(&path_in_target, &target)?
                        && !policy.is_updating_forced(&relative)?)
                {
                    write = false;
                }
            }
            if write {
                log::trace!("writing to {}", path_in_target.display());
                if let Some(parent) = path_in_target.parent() {
                    file_util::create_dir_all(parent)?;
                }
                let mut overwrite_target = file_util::create_file(&path_in_target)?;
                overwrite_target
                    .write_all(&file_contents)
                    .map_err(|e| TmcError::ZipWrite(path_in_target.clone(), e))?;
            }
        }
        unzip_paths.insert(
            path_in_target
                .canonicalize()
                .map_err(|e| TmcError::Canonicalize(path_in_target.clone(), e))?,
        );
    }

    // delete non-student files that were not in zip
    log::debug!("deleting non-student files not in zip");
    log::debug!("{:?}", unzip_paths);
    for entry in WalkDir::new(target).into_iter().filter_map(|e| e.ok()) {
        if !unzip_paths.contains(
            &entry
                .path()
                .canonicalize()
                .map_err(|e| TmcError::Canonicalize(entry.path().to_path_buf(), e))?,
        ) && (policy.is_updating_forced(entry.path())?
            || !policy.is_student_file(entry.path(), &target)?)
        {
            log::debug!("rm {} {}", entry.path().display(), target.display());
            if entry.path().is_dir() {
                // delete if empty
                if WalkDir::new(entry.path()).max_depth(1).into_iter().count() == 1 {
                    log::debug!("deleting empty directory {}", entry.path().display());
                    file_util::remove_dir_empty(entry.path())?;
                }
            } else {
                log::debug!("removing file {}", entry.path().display());
                file_util::remove_file(entry.path())?;
            }
        }
    }

    Ok(())
}

fn find_project_dir<R: Read + Seek>(zip_archive: &mut ZipArchive<R>) -> Result<PathBuf, TmcError> {
    let mut lowest_ipynb_dir = None::<PathBuf>;

    for i in 0..zip_archive.len() {
        let file = zip_archive.by_index(i)?;
        let file_path = Path::new(file.name());

        // directories may not have entries in the zip, e.g. it may only have
        // exercise/src/main... without an entry for src, so we need to check
        // the path components to find src
        let mut components = file_path.components().peekable();
        let mut collected = vec![];
        while let Some(component) = components.next() {
            if components.peek().is_none() {
                // do not inspect the last component,
                // they will have an entry that is
                // processed in the next step
                break;
            }

            let comp = component.as_os_str();
            if comp == "nbproject" || comp == "src" || comp == "test" {
                let path: PathBuf = collected.into_iter().collect();
                return Ok(path);
            }
            collected.push(comp);
        }

        let file_name = file_path.file_name().unwrap_or_default();
        if file.is_dir() && (file_name == "nbproject" || file_name == "src" || file_name == "test")
            || file.is_file()
                && (file_name == "pom.xml" || file_name == ".idea" || file_name == "Makefile")
        {
            let parent = file_path.parent().unwrap_or_else(|| Path::new(""));
            log::debug!("found project dir {}", parent.display());
            return Ok(parent.to_path_buf());
        }

        if file_path
            .extension()
            .map(|ext| ext == "ipynb")
            .unwrap_or_default()
        {
            let parent = file_path.parent().unwrap_or(Path::new("./"));
            if let Some(lowest_ipynb_dir) = lowest_ipynb_dir.as_mut() {
                if lowest_ipynb_dir.components().count() > parent.components().count() {
                    *lowest_ipynb_dir = parent.to_path_buf();
                }
            } else {
                lowest_ipynb_dir = Some(parent.to_path_buf());
            }
        }
    }
    if let Some(lowest_ipynb_dir) = lowest_ipynb_dir {
        Ok(lowest_ipynb_dir)
    } else {
        Err(TmcError::NoProjectDirInZip)
    }
}

fn contains_tmcnosubmit(entry: &DirEntry) -> bool {
    for entry in WalkDir::new(entry.path())
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_name() == ".tmcnosubmit" {
            log::debug!("contains .tmcnosubmit: {}", entry.path().display());
            return true;
        }
    }
    false
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashSet;
    use std::fs::{self, *};
    use tempfile::tempdir;
    use tmc_langs_framework::policy::EverythingIsStudentFilePolicy;

    fn init() {
        use log::*;
        use simple_logger::*;
        let _ = SimpleLogger::new().with_level(LevelFilter::Debug).init();
    }

    fn get_relative_file_paths(dir: &Path) -> HashSet<PathBuf> {
        WalkDir::new(dir)
            .into_iter()
            .map(|e| e.unwrap())
            .map(|e| e.into_path())
            .filter(|e| e.is_file())
            .map(|e| e.strip_prefix(dir).unwrap().to_path_buf())
            .collect()
    }

    #[test]
    fn zips() {
        init();

        let temp = tempdir().unwrap();
        let student_file_path = temp
            .path()
            .join("exercise-name/src/main/java/AdaLovelace.java");
        let missing_file_path = temp.path().join("exercise-name/pom.xml");
        fs::create_dir_all(student_file_path.parent().unwrap()).unwrap();
        File::create(student_file_path).unwrap();
        fs::create_dir_all(missing_file_path.parent().unwrap()).unwrap();
        File::create(missing_file_path).unwrap();

        let path = temp.path().join("exercise-name");
        let zipped = zip(EverythingIsStudentFilePolicy::new(&path).unwrap(), &path).unwrap();
        let mut archive = ZipArchive::new(Cursor::new(zipped)).unwrap();
        assert!(!archive.is_empty());
        for i in 0..archive.len() {
            log::debug!("{:?}", archive.by_index(i).unwrap().name());
        }
        assert!(archive
            .by_name("exercise-name/src/main/java/AdaLovelace.java")
            .is_ok());
        assert!(archive.by_name("exercise-name/pom.xml").is_ok());
    }

    #[test]
    fn unzips_simple() {
        init();

        let temp = tempdir().unwrap();
        let zip = file_util::open_file("tests/data/zip/module-trivial.zip").unwrap();
        unzip(
            EverythingIsStudentFilePolicy::new(temp.path()).unwrap(),
            zip,
            temp.path(),
        )
        .unwrap();

        let expected = get_relative_file_paths(Path::new("tests/data/zip/module-trivial"));
        let actual = get_relative_file_paths(temp.path());
        assert_eq!(expected, actual)
    }

    #[test]
    fn unzips_complex() {
        init();

        let temp = tempdir().unwrap();
        let zip = file_util::open_file("tests/data/zip/course-module-trivial.zip").unwrap();
        unzip(
            EverythingIsStudentFilePolicy::new(temp.path()).unwrap(),
            zip,
            temp.path(),
        )
        .unwrap();

        let expected = get_relative_file_paths(Path::new("tests/data/zip/module-trivial"));
        let actual = get_relative_file_paths(temp.path());
        assert_eq!(expected, actual)
    }

    #[test]
    fn no_src_entry() {
        init();

        let temp = tempdir().unwrap();
        let zip = file_util::open_file("tests/data/zip/no-src-entry.zip").unwrap();
        unzip(
            EverythingIsStudentFilePolicy::new(temp.path()).unwrap(),
            zip,
            temp.path(),
        )
        .unwrap();
        assert!(temp.path().join("src").exists());
    }

    #[cfg(windows)]
    #[test]
    fn windows_paths_get_converted() {
        let win_path = PathBuf::from(r"tests\data\dir");
        let zipped = zip(
            EverythingIsStudentFilePolicy::new(&win_path).unwrap(),
            &win_path,
        )
        .unwrap();
        let mut ziparch = ZipArchive::new(Cursor::new(zipped)).unwrap();
        assert!(ziparch.len() > 0);
        for i in 0..ziparch.len() {
            let file = ziparch.by_index(i).unwrap();
            assert!(file.name().chars().find(|c| c == &'\\').is_none())
        }
    }
}
