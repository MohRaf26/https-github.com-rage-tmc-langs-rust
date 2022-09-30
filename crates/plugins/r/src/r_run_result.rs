//! Struct modeling the test run results from R.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tmc_langs_framework::{RunResult, RunStatus, TestResult};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RRunResult {
    run_status: RRunStatus,
    backtrace: Vec<String>,
    test_results: Vec<RTestResult>,
}

impl From<RRunResult> for RunResult {
    fn from(r_run_result: RRunResult) -> RunResult {
        let mut logs = HashMap::new();
        if !r_run_result.backtrace.is_empty() {
            logs.insert(
                "compiler_output".to_string(),
                r_run_result
                    .backtrace
                    .into_iter()
                    .map(|s| format!("{}\n", s))
                    .collect(),
            );
        }
        let status = match r_run_result.run_status {
            RRunStatus::Success => {
                // check test results to determine the status
                let mut status = RunStatus::Passed;
                for test_result in &r_run_result.test_results {
                    if test_result.status != RTestStatus::Pass {
                        status = RunStatus::TestsFailed;
                    }
                }
                status
            }
            // todo: differentiate between different errors?
            _ => RunStatus::CompileFailed,
        };

        RunResult {
            status,
            test_results: r_run_result
                .test_results
                .into_iter()
                .map(|t| t.into())
                .collect(),
            logs,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum RRunStatus {
    Success,
    RunFailed,
    SourcingFailed,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
struct RTestResult {
    status: RTestStatus,
    name: String,
    message: String,
    backtrace: Vec<String>,
    points: Vec<String>,
}

impl From<RTestResult> for TestResult {
    fn from(r_test_result: RTestResult) -> TestResult {
        TestResult {
            name: r_test_result.name,
            successful: r_test_result.status == RTestStatus::Pass,
            points: r_test_result.points,
            message: r_test_result.message,
            exception: r_test_result.backtrace,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
enum RTestStatus {
    Pass,
    Fail,
}

#[cfg(test)]
mod test {
    use super::*;

    fn init() {
        use log::*;
        use simple_logger::*;
        let _ = SimpleLogger::new().with_level(LevelFilter::Debug).init();
    }

    #[test]
    fn converts() {
        init();

        let r_run_result = RRunResult {
            run_status: RRunStatus::Success,
            test_results: vec![],
            backtrace: vec![],
        };
        let run_result: RunResult = r_run_result.into();
        assert_eq!(run_result.status, RunStatus::Passed);
    }
}
