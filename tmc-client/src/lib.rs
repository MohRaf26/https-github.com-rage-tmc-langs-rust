#![deny(clippy::print_stdout, clippy::print_stderr, clippy::unwrap_used)]

//! Used to communicate with the TMC server. See the TmcClient struct for more details.
//!
//! ```rust,no_run
//! use tmc_client::TmcClient;
//!
//! let mut client = TmcClient::new("https://tmc.mooc.fi".parse().unwrap(), "some_client".to_string(), "some_version".to_string());
//! client.authenticate("client_name", "email".to_string(), "password".to_string());
//! let organizations = client.get_organizations();
//! ```
//!

mod error;
mod request;
mod response;
mod tmc_client;

pub use self::error::ClientError;
pub use self::request::FeedbackAnswer;
pub use self::response::{
    Course, CourseData, CourseDataExercise, CourseDataExercisePoint, CourseDetails, CourseExercise,
    Exercise, ExerciseDetails, ExercisesDetails, NewSubmission, Organization, Review, Submission,
    SubmissionFeedbackResponse, SubmissionFinished, SubmissionProcessingStatus, SubmissionStatus,
    User,
};
pub use self::tmc_client::{api_v8, ClientUpdateData, TmcClient, Token, UpdateResult};
pub use oauth2;
pub use tmc_langs_plugins::{Language, RunResult, StyleValidationResult, StyleValidationStrategy};
