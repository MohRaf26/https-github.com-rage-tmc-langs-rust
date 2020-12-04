//! Contains structs that model data related to exercises.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

/// A description of an exercise's test case.
#[derive(Debug, Deserialize, Serialize, JsonSchema, PartialEq, Eq, Hash)]
pub struct TestDesc {
    /// The full name of the test.
    ///
    /// If the language organises tests into suites or classes, it is customary
    /// to name the test as "class_name.method_name".
    pub name: String,
    /// The list of point names that passing this test may give.
    ///
    /// To obtain a point X, the user must pass all exercises that require point X.
    pub points: Vec<String>,
}

impl TestDesc {
    pub fn new(name: String, points: Vec<String>) -> Self {
        Self { name, points }
    }
}

/// The result of a single test case.
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct TestResult {
    pub name: String,
    pub successful: bool,
    pub points: Vec<String>,
    pub message: String,
    #[serde(default)]
    pub exception: Vec<String>,
}

/// A description of an exercise.
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct ExerciseDesc {
    /// The name of the exercise to be shown to the user.
    /// Does not necessarily match or even contain the directory name.
    pub name: String,
    /// Descriptions of the tests that will be run for this exercise.
    pub tests: Vec<TestDesc>,
}

impl ExerciseDesc {
    pub fn new(name: String, tests: Vec<TestDesc>) -> Self {
        Self { name, tests }
    }
}

/// The result of running an exercise's test suite against a submission.
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RunResult {
    /// The overall status of a test run.
    pub status: RunStatus,
    /// Whether each test passed and which points were awarded.
    pub test_results: Vec<TestResult>,
    /// Logs from the test run.
    /// The key may be an arbitrary string identifying the type of log.
    pub logs: HashMap<String, String>,
}

impl RunResult {
    pub fn new(
        status: RunStatus,
        test_results: Vec<TestResult>,
        logs: HashMap<String, String>,
    ) -> Self {
        Self {
            status,
            test_results,
            logs,
        }
    }
}

/// The overall status of a test run.
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RunStatus {
    /// The submission and tests compiled and all tests passed.
    Passed,
    /// The submission and tests compiled but some tests failed.
    TestsFailed,
    /// The submission or tests did not compile.
    // TODO: "The compiler error should be given in {@code logs[SpecialLogs.COMPILER_OUTPUT]}."
    CompileFailed,
    /// The submission compiled but testrun was interrupted.
    TestrunInterrupted,
    /// For when no other status seems suitable, or the language plugin has
    /// suffered an internal error.
    // TODO: "Details should be given in {@code logs[SpecialLogs.GENERIC_ERROR_MESSAGE]}.""
    GenericError,
}

/// Represents configuration based on which submission may be packaged.
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct ExercisePackagingConfiguration {
    /// Student folders or files which are copied from submission.
    pub student_file_paths: HashSet<PathBuf>,
    /// Exercise folders or files which are copied from exercise template or clone.
    pub exercise_file_paths: HashSet<PathBuf>,
}

impl ExercisePackagingConfiguration {
    pub fn new(
        student_file_paths: HashSet<PathBuf>,
        exercise_file_paths: HashSet<PathBuf>,
    ) -> Self {
        Self {
            student_file_paths,
            exercise_file_paths,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "UPPERCASE")]
pub enum Strategy {
    Fail,
    Warn,
    Disabled,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ValidationError {
    pub column: usize,
    pub line: usize,
    pub message: String,
    pub source_name: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ValidationResult {
    pub strategy: Strategy,
    pub validation_errors: Option<HashMap<PathBuf, Vec<ValidationError>>>,
}
