//! Contains a struct representing valgrind's log output.

use crate::error::MakeError;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::path::Path;
use tmc_langs_util::{file_util, FileError};

#[derive(Debug)]
pub struct ValgrindLog {
    pub header: (String, Vec<String>),
    pub errors: bool,
    pub results: Vec<ValgrindResult>,
}

impl ValgrindLog {
    /// Attempts to read and parse the log file at the given path.
    pub fn from(valgrind_log_path: &Path) -> Result<Self, MakeError> {
        // TODO: use parsing lib?
        log::debug!("parsing {}", valgrind_log_path.display());

        lazy_static! {
            static ref PID_REGEX: Regex = Regex::new(r#"==(?P<pid>\d+)=="#).unwrap();
            static ref ERR_REGEX: Regex =
                Regex::new(r#"== ERROR SUMMARY: (?P<error_count>\d+)"#).unwrap();
        }

        let valgrind_log_file = file_util::open_file(valgrind_log_path)?;
        let valgrind_log = BufReader::new(valgrind_log_file);

        let mut first_pid = None;
        let mut pid_info = HashMap::new();
        // parse all lines into a map of pid => ([lines of text], error count)
        for line in valgrind_log.lines() {
            let line = line.map_err(|e| FileError::FileRead(valgrind_log_path.to_path_buf(), e))?;
            let pid = match PID_REGEX.captures(&line) {
                Some(captures) => captures["pid"].to_string(),
                None => continue, // ignore lines without a PID
            };

            first_pid.get_or_insert(pid.clone());
            let info = pid_info.entry(pid).or_insert((vec![], 0));
            if let Some(captures) = ERR_REGEX.captures(&line) {
                let errors = captures["error_count"].parse::<u32>()?;
                info.1 = errors;
            }
            info.0.push(line);
        }

        let first_pid = match first_pid {
            Some(first_pid) => first_pid,
            None => return Err(MakeError::NoPidsInValgrindLogs),
        };
        let (header_log, _header_errors) = pid_info.remove(&first_pid).unwrap();

        let mut contains_errors = false;
        let mut results = vec![];
        for (pid, (log, errors)) in pid_info {
            let errors = errors > 0;
            contains_errors = contains_errors || errors;
            results.push(ValgrindResult { pid, errors, log })
        }

        let log = ValgrindLog {
            header: (first_pid, header_log),
            errors: contains_errors,
            results,
        };

        log::trace!("parsed {:#?}", log);
        Ok(log)
    }
}

#[derive(Debug)]
pub struct ValgrindResult {
    pub pid: String,
    pub errors: bool,
    pub log: Vec<String>,
}

#[cfg(test)]
mod test {
    use super::*;

    fn init() {
        use log::*;
        use simple_logger::*;
        let _ = SimpleLogger::new().with_level(LevelFilter::Debug).init();
    }

    fn file_to(
        target_dir: impl AsRef<std::path::Path>,
        target_relative: impl AsRef<std::path::Path>,
        contents: impl AsRef<[u8]>,
    ) -> std::path::PathBuf {
        let target = target_dir.as_ref().join(target_relative);
        if let Some(parent) = target.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        std::fs::write(&target, contents.as_ref()).unwrap();
        target
    }

    #[test]
    fn parses_errors() {
        init();

        let temp_dir = tempfile::tempdir().unwrap();
        let file = file_to(
            &temp_dir,
            "file",
            r#"
==1234== 
==1234== stuff
==1234== 
==1234== ERROR SUMMARY: 11

==2345==
==2345== stuff
==2345==
==2345== ERROR SUMMARY: 22
"#,
        );

        let valgrind_log = ValgrindLog::from(&file).unwrap();
        log::debug!("{:#?}", valgrind_log);
        assert!(valgrind_log.errors);
    }

    #[test]
    fn parses_no_errors() {
        init();

        let temp_dir = tempfile::tempdir().unwrap();
        let file = file_to(
            &temp_dir,
            "file",
            r#"
==1234== 
==1234== stuff
==1234== 
==1234== ERROR SUMMARY: 0

==2345==
==2345== stuff
==2345==
"#,
        );

        let valgrind_log = ValgrindLog::from(&file).unwrap();
        log::debug!("{:#?}", valgrind_log);
        assert!(!valgrind_log.errors);
    }
}
