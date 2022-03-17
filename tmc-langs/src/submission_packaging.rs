//! Submission packaging.

use crate::{
    data::{OutputFormat, TmcParams},
    error::LangsError,
};
use once_cell::sync::Lazy;
use std::{
    io::{Cursor, Write},
    path::{Path, PathBuf},
    sync::Mutex,
};
use tmc_langs_plugins::Plugin;
use tmc_langs_util::{file_util, FileError};
use walkdir::WalkDir;
use zip::{write::FileOptions, ZipArchive, ZipWriter};

static MUTEX: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

/// Note: Used by tmc-server. Prepares a submission for further processing.
/// The clone path is assumed to be a directory with the exercise name as the directory name,
/// and the course name as its parent, ex. "anything/some_course/some_exercise"
pub fn prepare_submission(
    zip_path: &Path,
    target_path: &Path,
    toplevel_dir_name: Option<String>,
    tmc_params: TmcParams,
    stub_clone_path: &Path,
    stub_zip_path: Option<&Path>,
    output_format: OutputFormat,
) -> Result<(), LangsError> {
    // workaround for unknown issues when prepare_submission is ran multiple times in parallel
    let _m = MUTEX.lock().map_err(|_| LangsError::MutexError)?;
    log::debug!("preparing submission for {}", zip_path.display());

    let plugin = tmc_langs_plugins::get_language_plugin(stub_clone_path)?;

    let extract_dest = tempfile::tempdir().map_err(LangsError::TempDir)?;
    let extract_dest_path = extract_dest.path().to_path_buf();

    // extract base
    log::debug!("extracting stub");
    let ignore_list = [
        ".DS_Store",
        "desktop.ini",
        "Thumbs.db",
        ".directory",
        "__MACOSX",
    ];
    if let Some(stub_zip_path) = stub_zip_path {
        // if defined, extract and use as the base
        unzip_with_filter(
            &plugin,
            stub_zip_path,
            |path| {
                path.components().any(|c| {
                    c.as_os_str()
                        .to_str()
                        .map(|s| ignore_list.contains(&s))
                        .unwrap_or_default()
                })
            },
            &extract_dest_path,
        )?;
    } else {
        // else, copy clone path
        for entry in WalkDir::new(stub_clone_path).min_depth(1) {
            let entry = entry?;

            if entry.path().components().any(|c| {
                c.as_os_str()
                    .to_str()
                    .map(|s| ignore_list.contains(&s))
                    .unwrap_or_default()
            }) {
                // path component on ignore list
                continue;
            }

            let relative_path = entry
                .path()
                .strip_prefix(stub_clone_path)
                .expect("entry is in stub clone path");
            let target_path = extract_dest_path.join(relative_path);
            if entry.path().is_file() {
                file_util::copy(entry.path(), target_path)?;
            } else if entry.path().is_dir() {
                file_util::create_dir(target_path)?;
            }
        }
    }

    // extract student files from submission over base
    log::debug!("extracting student files");
    let file = file_util::open_file(zip_path)?;
    plugin.extract_student_files(file, &extract_dest_path)?;

    // extract ide files
    log::debug!("extracting ide files");
    let ide_files = [
        // netbeans
        "nbproject",
        // eclipse
        ".classpath",
        ".project",
        ".settings",
        // idea
        ".idea",
    ];
    unzip_with_filter(
        &plugin,
        zip_path,
        |path| {
            path.components().all(|c| {
                c.as_os_str()
                    .to_str()
                    .map(|s| !ide_files.contains(&s))
                    .unwrap_or_default()
            })
        },
        &extract_dest_path,
    )?;

    // write tmc params
    if tmc_params.0.is_empty() {
        log::debug!("no tmc params to write");
    } else {
        log::debug!("writing .tmcparams");
        let tmc_params_path = extract_dest_path.join(".tmcparams");
        let mut tmc_params_file = file_util::create_file(&tmc_params_path)?;
        for (key, value) in tmc_params.0 {
            // todo handle arrays, shell escapes
            let export = format!("export {}={}\n", key, value);
            log::debug!("{}", export);
            tmc_params_file
                .write_all(export.as_bytes())
                .map_err(|e| FileError::FileWrite(tmc_params_path.clone(), e))?;
        }
    }

    // make archive
    log::debug!("creating submission archive");
    let exercise_name = stub_clone_path.file_name();
    let course_name = stub_clone_path.parent().and_then(Path::file_name);
    let prefix = match (toplevel_dir_name, course_name, exercise_name) {
        (Some(toplevel_dir_name), Some(course_name), Some(exercise_name)) => {
            Path::new(&toplevel_dir_name)
                .join(course_name)
                .join(exercise_name)
        }
        (None, Some(course_name), Some(exercise_name)) => {
            Path::new(course_name).join(exercise_name)
        }
        _ => {
            log::warn!(
                "was not able to find exercise and/or course name from clone path {}",
                stub_clone_path.display()
            );
            PathBuf::from("")
        }
    };
    let archive_file = file_util::create_file(target_path)?;
    match output_format {
        OutputFormat::Tar => {
            let mut archive = tar::Builder::new(archive_file);
            log::debug!(
                "appending \"{}\" at \"{}\"",
                extract_dest_path.display(),
                prefix.display()
            );
            archive
                .append_dir_all(prefix, &extract_dest_path)
                .map_err(|e| LangsError::TarAppend(extract_dest_path, e))?;
        }
        OutputFormat::Zip => {
            let mut archive = ZipWriter::new(archive_file);
            for entry in WalkDir::new(&extract_dest_path).into_iter().skip(1) {
                let entry = entry?;
                let entry_path = entry.path();
                let stripped = prefix.join(
                    entry_path
                        .strip_prefix(&extract_dest_path)
                        .expect("the entry is inside dest"),
                );
                log::debug!(
                    "adding {} to zip at {}",
                    entry_path.display(),
                    stripped.display()
                );
                if entry_path.is_dir() {
                    archive.add_directory(
                        stripped.to_string_lossy(),
                        FileOptions::default().unix_permissions(0o755),
                    )?;
                } else {
                    archive.start_file(
                        stripped.to_string_lossy(),
                        FileOptions::default().unix_permissions(0o755),
                    )?;
                    let mut file = file_util::open_file(entry_path)?;
                    std::io::copy(&mut file, &mut archive)
                        .map_err(|e| LangsError::TarAppend(entry_path.to_path_buf(), e))?;
                }
            }
            archive.finish()?;
        }
        OutputFormat::TarZstd => {
            let buf = Cursor::new(vec![]);
            let mut archive = tar::Builder::new(buf);
            log::debug!(
                "appending \"{}\" at \"{}\"",
                extract_dest_path.display(),
                prefix.display()
            );
            archive
                .append_dir_all(prefix, &extract_dest_path)
                .map_err(|e| LangsError::TarAppend(extract_dest_path, e))?;
            archive.finish().map_err(LangsError::TarFinish)?;
            let mut tar = archive.into_inner().map_err(LangsError::TarIntoInner)?;
            tar.set_position(0); // reset the cursor
            zstd::stream::copy_encode(tar, archive_file, 0).map_err(LangsError::Zstd)?;
        }
    }
    Ok(())
}

fn unzip_with_filter<F: Fn(&Path) -> bool>(
    plugin: &Plugin,
    zip_path: &Path,
    exclude_filter: F,
    dest: &Path,
) -> Result<(), LangsError> {
    let file = file_util::open_file(zip_path)?;
    let mut zip = ZipArchive::new(file)?;
    let project_dir_in_stub = plugin.find_project_dir_in_zip(&mut zip)?;
    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;
        if file.is_file() {
            if let Ok(path) = file
                .enclosed_name()
                .ok_or_else(|| LangsError::InvalidZipPath {
                    zip_path: zip_path.to_path_buf(),
                    file_path: file.name().to_string(),
                })?
                .strip_prefix(&project_dir_in_stub)
            {
                if exclude_filter(path) {
                    // path component on ignore list
                    continue;
                };
                let target = dest.join(path);
                file_util::read_to_file(&mut file, &target)?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
#[cfg(target_os = "linux")] // no maven plugin on other OS
#[allow(clippy::unwrap_used)]
mod test {
    use super::*;
    use std::{fs, path::PathBuf};
    use tempfile::TempDir;
    use walkdir::WalkDir;

    const MAVEN_CLONE: &str = "tests/data/some_course/MavenExercise";
    const MAVEN_ZIP: &str = "tests/data/MavenExercise.zip";

    const MAKE_CLONE: &str = "tests/data/some_course/MakeExercise";
    const MAKE_ZIP: &str = "tests/data/MakeExercise.zip";

    const PYTHON_CLONE: &str = "tests/data/some_course/PythonExercise";
    const PYTHON_ZIP: &str = "tests/data/PythonExercise.zip";

    fn init() {
        use log::*;
        use simple_logger::*;
        let _ = SimpleLogger::new()
            .with_level(LevelFilter::Debug)
            .with_module_level("j4rs", LevelFilter::Warn)
            .init();
    }

    fn generic_submission(clone: &str, zip: &str) -> (TempDir, PathBuf) {
        let temp = tempfile::tempdir().unwrap();
        let output_archive = temp.path().join("output.tar");

        let mut tmc_params = TmcParams::new();
        tmc_params.insert_string("param_one", "value_one").unwrap();
        tmc_params
            .insert_array("param_two", vec!["value_two", "value_three"])
            .unwrap();
        prepare_submission(
            Path::new(zip),
            &output_archive,
            None,
            tmc_params,
            Path::new(clone),
            None,
            OutputFormat::Tar,
        )
        .unwrap();
        assert!(output_archive.exists());

        let output_file = file_util::open_file(&output_archive).unwrap();
        let mut archive = tar::Archive::new(output_file);
        let output_extracted = temp.path().join("output");
        archive.unpack(&output_extracted).unwrap();
        for entry in WalkDir::new(temp.path()) {
            log::debug!("file {}", entry.unwrap().path().display());
        }
        (temp, output_extracted)
    }

    #[test]
    fn package_has_expected_files() {
        init();
        let (_temp, output) = generic_submission(MAVEN_CLONE, MAVEN_ZIP);
        // expected files
        assert!(output
            .join("some_course/MavenExercise/src/main/java/SimpleStuff.java")
            .exists());
        assert!(output
            .join("some_course/MavenExercise/src/test/java/SimpleTest.java")
            .exists());
        assert!(output
            .join("some_course/MavenExercise/src/test/java/SimpleHiddenTest.java")
            .exists());
        assert!(output.join("some_course/MavenExercise/pom.xml").exists());
    }

    #[test]
    fn package_doesnt_have_unwanted_files() {
        init();
        let (_temp, output) = generic_submission(MAVEN_CLONE, MAVEN_ZIP);

        // files that should not be included
        assert!(!output.join("some_course/MavenExercise/__MACOSX").exists());
        assert!(!output
            .join("some_course/MavenExercise/src/test/java/MadeUpTest.java")
            .exists());
    }

    #[test]
    fn modified_test_file_not_included_in_package() {
        init();
        let (_temp, output) = generic_submission(MAVEN_CLONE, MAVEN_ZIP);

        // submission zip has a test file with the text MODIFIED...
        let zipfile = file_util::open_file(MAVEN_ZIP).unwrap();
        let mut zip = zip::ZipArchive::new(zipfile).unwrap();
        let mut modified = zip
            .by_name("MavenExercise/src/test/java/SimpleTest.java")
            .unwrap();
        let mut writer: Vec<u8> = vec![];
        std::io::copy(&mut modified, &mut writer).unwrap();
        let contents = String::from_utf8(writer).unwrap();
        assert!(contents.contains("MODIFIED"));
        // the text should not be in the package
        let test_file = fs::read_to_string(dbg!(
            output.join("some_course/MavenExercise/src/test/java/SimpleTest.java")
        ))
        .unwrap();
        assert!(!test_file.contains("MODIFIED"));
    }

    #[test]
    fn writes_tmcparams() {
        init();
        let (_temp, output) = generic_submission(MAVEN_CLONE, MAVEN_ZIP);

        let param_file = output.join("some_course/MavenExercise/.tmcparams");
        assert!(param_file.exists());
        let conts = fs::read_to_string(param_file).unwrap();
        log::debug!("tmcparams {}", conts);
        let lines: Vec<_> = conts.lines().collect();
        assert_eq!(lines.len(), 2);
        assert!(lines.contains(&"export param_one=value_one"));
        assert!(lines.contains(&"export param_two=( value_two value_three )"));
    }

    #[test]
    fn packages_with_toplevel_dir_name() {
        init();

        let temp = tempfile::tempdir().unwrap();
        let output = temp.path().join("output.tar");

        assert!(!output.exists());
        prepare_submission(
            Path::new(MAVEN_ZIP),
            &output,
            Some("toplevel".to_string()),
            TmcParams::new(),
            Path::new(MAVEN_CLONE),
            None,
            OutputFormat::Tar,
        )
        .unwrap();
        assert!(output.exists());

        let output = file_util::open_file(output).unwrap();
        let mut archive = tar::Archive::new(output);
        let output = temp.path().join("output");
        archive.unpack(&output).unwrap();
        for entry in WalkDir::new(temp.path()) {
            log::debug!("{}", entry.unwrap().path().display());
        }
        assert!(output
            .join("toplevel/some_course/MavenExercise/src/test/java/SimpleHiddenTest.java")
            .exists());
        assert!(output
            .join("toplevel/some_course/MavenExercise/pom.xml")
            .exists());
    }

    #[test]
    fn packages_with_output_zstd() {
        init();

        let temp = tempfile::tempdir().unwrap();
        let output = temp.path().join("output.tar.zst");

        assert!(!output.exists());
        prepare_submission(
            Path::new(MAVEN_ZIP),
            &output,
            None,
            TmcParams::new(),
            Path::new(MAVEN_CLONE),
            None,
            OutputFormat::TarZstd,
        )
        .unwrap();
        assert!(output.exists());

        let output = file_util::open_file(output).unwrap();
        let output = std::io::Cursor::new(zstd::decode_all(output).unwrap());
        let mut archive = tar::Archive::new(output);
        let output = temp.path().join("output");
        archive.unpack(&output).unwrap();
        for entry in WalkDir::new(temp.path()) {
            log::debug!("{}", entry.unwrap().path().display());
        }
        assert!(output
            .join("some_course/MavenExercise/src/test/java/SimpleHiddenTest.java")
            .exists());
        assert!(output.join("some_course/MavenExercise/pom.xml").exists());
    }

    #[test]
    fn packages_with_output_zip() {
        init();

        let temp = tempfile::tempdir().unwrap();
        let output = temp.path().join("output.zip");

        assert!(!output.exists());
        prepare_submission(
            Path::new(MAVEN_ZIP),
            &output,
            None,
            TmcParams::new(),
            Path::new(MAVEN_CLONE),
            None,
            OutputFormat::Zip,
        )
        .unwrap();
        assert!(output.exists());

        let output = file_util::open_file(output).unwrap();
        let mut archive = zip::ZipArchive::new(output).unwrap();
        archive
            .by_name("some_course/MavenExercise/src/test/java/SimpleHiddenTest.java")
            .unwrap();
    }

    #[test]
    fn packages_with_toplevel_dir_and_output_zip() {
        init();

        let temp = tempfile::tempdir().unwrap();
        let output = temp.path().join("output.zip");

        assert!(!output.exists());
        prepare_submission(
            Path::new(MAVEN_ZIP),
            &output,
            Some("toplevel".to_string()),
            TmcParams::new(),
            Path::new(MAVEN_CLONE),
            None,
            OutputFormat::Zip,
        )
        .unwrap();
        assert!(output.exists());

        let output = file_util::open_file(output).unwrap();
        let mut archive = zip::ZipArchive::new(output).unwrap();
        archive
            .by_name("toplevel/some_course/MavenExercise/src/test/java/SimpleHiddenTest.java")
            .unwrap();
        archive
            .by_name("toplevel/some_course/MavenExercise/pom.xml")
            .unwrap();
    }

    #[test]
    fn package_with_stub_tests() {
        init();

        let temp = tempfile::tempdir().unwrap();
        let output_arch = temp.path().join("output.tar");

        assert!(!output_arch.exists());
        prepare_submission(
            Path::new(MAVEN_ZIP),
            &output_arch,
            None,
            TmcParams::new(),
            Path::new(MAVEN_CLONE),
            Some(Path::new("tests/data/MavenStub.zip")),
            OutputFormat::Tar,
        )
        .unwrap();
        assert!(output_arch.exists());

        let output_file = file_util::open_file(&output_arch).unwrap();
        let mut archive = tar::Archive::new(output_file);
        let output_extracted = temp.path().join("output");
        archive.unpack(&output_extracted).unwrap();
        for entry in WalkDir::new(temp.path()) {
            log::debug!("{}", entry.unwrap().path().display());
        }

        // visible tests included, hidden test isn't
        assert!(output_extracted
            .join("some_course/MavenExercise/src/test/java/SimpleTest.java")
            .exists());
        assert!(!output_extracted
            .join("some_course/MavenExercise/src/test/java/SimpleHiddenTest.java")
            .exists());
    }

    #[test]
    fn prepare_make_submission() {
        init();
        let (_temp, output) = generic_submission(MAKE_CLONE, MAKE_ZIP);

        // expected files
        assert!(output.join("some_course/MakeExercise/src/main.c").exists());
        assert!(output
            .join("some_course/MakeExercise/test/test_source.c")
            .exists());
        assert!(output.join("some_course/MakeExercise/Makefile").exists());
    }

    #[test]
    fn prepare_langs_submission() {
        init();
        let (_temp, output) = generic_submission(PYTHON_CLONE, PYTHON_ZIP);

        // expected files
        assert!(output
            .join("some_course/PythonExercise/src/__main__.py")
            .exists());
        assert!(output
            .join("some_course/PythonExercise/test/test_greeter.py")
            .exists());
        // assert!(output.join("tmc/points.py").exists()); // not included?
        assert!(output
            .join("some_course/PythonExercise/__init__.py")
            .exists());
    }
}
