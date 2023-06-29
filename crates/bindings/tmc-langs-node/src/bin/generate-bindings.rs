//! Exports TypeScript bindings.

use std::fs::File;

fn main() {
    let mut file = File::create(concat!(env!("CARGO_MANIFEST_DIR"), "/ts/generated.d.ts")).unwrap();
    ts_rs::export_to! {
        &mut file,
        // checkstyle
        tmc_langs::StyleValidationResult,
        tmc_langs::StyleValidationError,
        tmc_langs::StyleValidationStrategy,
        // getExercisePackagingConfiguration
        tmc_langs::ExercisePackagingConfiguration,
        // listLocalCourseExercises
        tmc_langs::LocalExercise,
        // prepareSubmission
        tmc_langs::Compression,
        // refreshCourse
        tmc_langs::RefreshData,
        tmc_langs::RefreshExercise,
        tmc_langs::TmcProjectYml,
        tmc_langs::PythonVer,
        // runTests
        tmc_langs::RunResult,
        tmc_langs::RunStatus,
        tmc_langs::TestResult,
        // scanExercise
        tmc_langs::ExerciseDesc,
        tmc_langs::TestDesc,
        // checkExerciseUpdates
        tmc_langs::UpdatedExercise,
        // downloadOrUpdateCourseExercises
        tmc_langs::DownloadOrUpdateCourseExercisesResult,
        tmc_langs::ExerciseDownload,
        // getCourseData
        tmc_langs::CombinedCourseData,
        // getCourseData
        tmc_langs::tmc::response::CourseDetails,
        tmc_langs::tmc::response::Exercise,
        // getCourseExercises
        tmc_langs::tmc::response::CourseExercise,
        tmc_langs::tmc::response::ExercisePoint,
        // getCourseSettings
        // getCourses
        tmc_langs::tmc::response::CourseData,
        // getExerciseDetails
        tmc_langs::tmc::response::ExerciseDetails,
        tmc_langs::tmc::response::ExerciseSubmission,
        // getExerciseSubmissions
        tmc_langs::tmc::response::Submission,
        // getExerciseUpdates
        tmc_langs::tmc::UpdateResult,
        // getOrganization
        // getOrganizations
        tmc_langs::tmc::response::Organization,
        // getUnreadReviews
        tmc_langs::tmc::response::Review,
        // paste
        // requestCodeReview
        tmc_langs::tmc::response::NewSubmission,
        // sendFeedback
        tmc_langs::tmc::response::SubmissionFeedbackResponse,
        tmc_langs::tmc::response::SubmissionStatus,
        // submit
        // waitForSubmission
        tmc_langs::tmc::response::SubmissionFinished,
        tmc_langs::tmc::response::TestCase,
        tmc_langs::tmc::response::SubmissionFeedbackQuestion,
        tmc_langs::tmc::response::SubmissionFeedbackKind,
        // listSettings
        tmc_langs::TmcConfig,
    }
    .unwrap()
}
