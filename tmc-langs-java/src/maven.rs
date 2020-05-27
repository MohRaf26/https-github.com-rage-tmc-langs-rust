pub mod policy;

use super::{error::JavaPluginError, plugin::JavaPlugin, CompileResult, TestRun, SEPARATOR};
use isolang::Language;
use j4rs::Jvm;
use policy::MavenStudentFilePolicy;
use std::fs;
use std::path::Path;
use std::process::Command;
use tmc_langs_abstraction::ValidationResult;
use tmc_langs_framework::{
    domain::{ExerciseDesc, RunResult},
    plugin::LanguagePlugin,
    policy::StudentFilePolicy,
    Error,
};

pub struct MavenPlugin {
    jvm: Jvm,
}

impl MavenPlugin {
    pub fn new() -> Result<Self, JavaPluginError> {
        let jvm = crate::instantiate_jvm()?;
        Ok(Self { jvm })
    }
}

impl LanguagePlugin for MavenPlugin {
    fn get_plugin_name(&self) -> &str {
        "apache-maven"
    }

    fn check_code_style(&self, path: &Path, locale: Language) -> Option<ValidationResult> {
        self.run_checkstyle(&locale, path)
    }

    fn scan_exercise(&self, path: &Path, exercise_name: String) -> Result<ExerciseDesc, Error> {
        if !self.is_exercise_type_correct(path) {
            return JavaPluginError::InvalidExercise.into();
        }

        let compile_result = self.build(path)?;
        self.scan_exercise_with_compile_result(path, exercise_name, compile_result)
    }

    fn run_tests(&self, project_root_path: &Path) -> Result<RunResult, Error> {
        self.run_java_tests(project_root_path)
    }

    fn is_exercise_type_correct(&self, path: &Path) -> bool {
        path.join("pom.xml").exists()
    }

    fn get_student_file_policy(&self, project_path: &Path) -> Box<dyn StudentFilePolicy> {
        Box::new(MavenStudentFilePolicy::new(project_path.to_path_buf()))
    }

    fn clean(&self, path: &Path) -> Result<(), Error> {
        log::info!("Cleaning maven project at {}", path.display());

        let output = Command::new("mvn")
            .current_dir(path)
            .arg("clean")
            .output()?;

        log::debug!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        log::debug!("stderr: {}", String::from_utf8_lossy(&output.stderr));

        if !output.status.success() {
            return JavaPluginError::FailedCommand("mvn").into();
        }

        Ok(())
    }
}

impl JavaPlugin for MavenPlugin {
    const TEST_DIR: &'static str = "src";

    fn jvm(&self) -> &Jvm {
        &self.jvm
    }

    fn get_project_class_path(&self, path: &Path) -> Result<String, Error> {
        log::info!("Building classpath for maven project at {}", path.display());

        let temp = tempfile::tempdir()?;
        let class_path_file = temp.path().join("cp.txt");

        let output_arg = format!("-Dmdep.outputFile={}", class_path_file.display());
        let output = Command::new("mvn")
            .current_dir(path)
            .arg("dependency:build-classpath")
            .arg(output_arg)
            .output()?;

        log::debug!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        log::debug!("stderr: {}", String::from_utf8_lossy(&output.stderr));

        if !output.status.success() {
            return JavaPluginError::FailedCommand("mvn").into();
        }

        let class_path = fs::read_to_string(class_path_file)?;
        if class_path.is_empty() {
            return JavaPluginError::NoMvnClassPath.into();
        }

        let mut class_path: Vec<String> = vec![class_path];
        class_path.push(path.join("target/classes").to_string_lossy().into_owned());
        class_path.push(
            path.join("target/test-classes")
                .to_string_lossy()
                .into_owned(),
        );

        Ok(class_path.join(SEPARATOR))
    }

    fn build(&self, project_root_path: &Path) -> Result<CompileResult, Error> {
        log::info!("Building maven project at {}", project_root_path.display());

        let output = Command::new("mvn")
            .current_dir(project_root_path)
            .arg("clean")
            .arg("compile")
            .arg("test-compile")
            .output()?;

        log::debug!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        log::debug!("stderr: {}", String::from_utf8_lossy(&output.stderr));

        if !output.status.success() {
            return JavaPluginError::FailedCommand("mvn").into();
        }

        Ok(CompileResult {
            status_code: output.status,
            stdout: output.stdout,
            stderr: output.stderr,
        })
    }

    fn create_run_result_file(
        &self,
        path: &Path,
        _compile_result: CompileResult,
    ) -> Result<TestRun, Error> {
        log::info!("Running tests for maven project at {}", path.display());

        let output = Command::new("mvn")
            .current_dir(path)
            .arg("fi.helsinki.cs.tmc:tmc-maven-plugin:1.12:test")
            .output()?;

        log::debug!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        log::debug!("stderr: {}", String::from_utf8_lossy(&output.stderr));

        if !output.status.success() {
            return JavaPluginError::FailedCommand("mvn").into();
        }

        Ok(TestRun {
            test_results: path.join("target/test_output.txt"),
            stdout: output.stdout,
            stderr: output.stderr,
        })
    }
}

#[cfg(test)]
mod test {
    use super::super::{TestCase, TestCaseStatus};
    use super::*;
    use tempfile::{tempdir, TempDir};
    use tmc_langs_abstraction::Strategy;
    use walkdir::WalkDir;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    fn copy_test_dir(path: &str) -> TempDir {
        let path = Path::new(path);

        let temp = tempdir().unwrap();
        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            let target = temp.path().join(entry.path().strip_prefix(path).unwrap());
            if entry.path().is_dir() {
                log::debug!("creating dirs {}", entry.path().display());
                fs::create_dir_all(target).unwrap();
            } else {
                log::debug!(
                    "copy from {} to {}",
                    entry.path().display(),
                    target.display()
                );
                fs::copy(entry.path(), target).unwrap();
            }
        }
        temp
    }

    #[test]
    fn gets_project_class_path() {
        init();

        let temp_dir = copy_test_dir("tests/data/maven_exercise");
        let test_path = temp_dir.path();
        let plugin = MavenPlugin::new().unwrap();
        let class_path = plugin.get_project_class_path(test_path).unwrap();
        assert!(class_path.contains("/junit/"));
    }

    #[test]
    fn builds() {
        init();

        use std::path::PathBuf;
        println!("{}", PathBuf::from(".").canonicalize().unwrap().display());

        let temp_dir = copy_test_dir("tests/data/maven_exercise");
        let test_path = temp_dir.path();
        let plugin = MavenPlugin::new().unwrap();
        let compile_result = plugin.build(test_path).unwrap();
        assert!(compile_result.status_code.success());
    }

    #[test]
    fn creates_run_result_file() {
        init();

        let temp_dir = copy_test_dir("tests/data/maven_exercise");
        let test_path = temp_dir.path();
        let plugin = MavenPlugin::new().unwrap();
        let compile_result = plugin.build(test_path).unwrap();
        let test_run = plugin
            .create_run_result_file(test_path, compile_result)
            .unwrap();
        let test_result: Vec<TestCase> =
            serde_json::from_str(&fs::read_to_string(test_run.test_results).unwrap()).unwrap();
        let test_case = &test_result[0];

        assert_eq!(test_case.class_name, "fi.helsinki.cs.maventest.AppTest");
        assert_eq!(test_case.point_names, ["maven-exercise"]);
        assert_eq!(test_case.status, TestCaseStatus::Failed);
        let message = test_case.message.as_ref().unwrap();
        assert!(message.starts_with("ComparisonFailure"));

        let exception = test_case.exception.as_ref().unwrap();
        assert_eq!(exception.class_name, "org.junit.ComparisonFailure");
        assert!(exception.message.starts_with("expected"));
        let stack_trace = &exception.stack_trace[0];
        assert_eq!(stack_trace.declaring_class, "org.junit.Assert");
        assert_eq!(stack_trace.file_name, "Assert.java");
        assert_eq!(stack_trace.line_number, 115);
        assert_eq!(stack_trace.method_name, "assertEquals");
    }

    #[test]
    fn scans_exercise() {
        init();

        let temp_dir = copy_test_dir("tests/data/maven_exercise");
        let test_path = temp_dir.path();
        let plugin = MavenPlugin::new().unwrap();
        let exercises = plugin
            .scan_exercise(&test_path, "test".to_string())
            .unwrap();
        assert_eq!(exercises.name, "test");
        assert_eq!(exercises.tests.len(), 1);
        assert_eq!(
            exercises.tests[0].name,
            "fi.helsinki.cs.maventest.AppTest trol"
        );
        assert_eq!(exercises.tests[0].points, ["maven-exercise"]);
    }

    #[test]
    fn runs_checkstyle() {
        init();

        let temp_dir = copy_test_dir("tests/data/maven_exercise");
        let test_path = temp_dir.path();
        let plugin = MavenPlugin::new().unwrap();
        let checkstyle_result = plugin
            .check_code_style(test_path, Language::from_639_3("fin").unwrap())
            .unwrap();

        assert_eq!(checkstyle_result.strategy, Strategy::Fail);
        let validation_errors = checkstyle_result.validation_errors.unwrap();
        let errors = validation_errors
            .get(Path::new("fi/helsinki/cs/maventest/App.java"))
            .unwrap();
        assert_eq!(errors.len(), 1);
        let error = &errors[0];
        assert_eq!(error.column, 0);
        assert_eq!(error.line, 4);
        assert!(error.message.starts_with("Sisennys väärin"));
        assert_eq!(
            error.source_name,
            "com.puppycrawl.tools.checkstyle.checks.indentation.IndentationCheck"
        );
    }
}
