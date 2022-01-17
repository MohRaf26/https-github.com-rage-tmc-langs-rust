//! Struct representing the results of a Check test run.

use serde::Deserialize;
use std::collections::HashMap;
use tmc_langs_framework::{RunResult, RunStatus, TestResult};

#[derive(Debug, Deserialize)]
pub struct CheckLog {
    pub datetime: String,
    #[serde(rename = "suite")]
    pub test_suites: Vec<TestSuite>,
    pub duration: String,
}

impl CheckLog {
    /// Converts the log into a RunResult. The point map should contain a mapping from test.id to a list of points, e.g.
    /// "test_one" => ["1.1", "1.2"].
    pub fn into_run_result(
        self,
        mut point_map: HashMap<String, Vec<String>>,
        logs: HashMap<String, String>,
    ) -> RunResult {
        let mut status = RunStatus::Passed;
        let mut test_results = vec![];

        for suite in self.test_suites {
            for test in suite.tests {
                let successful = test.result == "success";
                if !successful {
                    status = RunStatus::TestsFailed;
                }

                let points = point_map.remove(&test.id).unwrap_or_default();
                let exceptions = vec![];
                test_results.push(TestResult {
                    name: test.description,
                    successful,
                    points,
                    message: test.message,
                    exception: exceptions,
                });
            }
        }
        RunResult {
            status,
            test_results,
            logs,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct TestSuite {
    pub title: String,
    #[serde(rename = "test")]
    pub tests: Vec<Test>,
}

#[derive(Debug, Deserialize)]
pub struct Test {
    pub result: String,
    pub path: String,
    #[serde(rename = "fn")]
    pub function: String,
    pub id: String,
    pub iteration: String,
    pub duration: String,
    pub description: String,
    pub message: String,
}
