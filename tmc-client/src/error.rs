//! The client error type.

use reqwest::{Method, StatusCode};
use thiserror::Error;
use tmc_langs_util::FileError;
use url::Url;

type TokenError = oauth2::RequestTokenError<
    oauth2::reqwest::Error<reqwest::Error>,
    oauth2::StandardErrorResponse<oauth2::basic::BasicErrorResponseType>,
>;

/// The main error type for tmc-client.
#[derive(Debug, Error)]
pub enum ClientError {
    // Arc
    #[error("Tried to mutate client while it was borrowed")]
    ArcBorrowed,

    // file IO
    #[error("Failed to create temporary file")]
    TempFile(#[source] std::io::Error),

    // network
    #[error("HTTP error {status} for {url}: {error}. Obsolete client: {obsolete_client}")]
    HttpError {
        url: Url,
        status: StatusCode,
        error: String,
        obsolete_client: bool,
    },
    #[error("Connection error trying to {0} {1}")]
    ConnectionError(Method, Url, #[source] reqwest::Error),
    #[error("OAuth2 password exchange error")]
    Token(#[source] Box<TokenError>),
    #[error("OAuth2 unexpected token response: {0}")]
    TokenParse(String, #[source] serde_json::error::Error),
    #[error("Failed to parse as URL: {0}")]
    UrlParse(String, #[source] url::ParseError),
    #[error("Failed to write response")]
    HttpWriteResponse(#[source] reqwest::Error),
    #[error("Failed to deserialize response from {0} as JSON")]
    HttpJsonResponse(Url, #[source] reqwest::Error),

    #[error("Failed to download some exercises")]
    IncompleteDownloadResult {
        downloaded: Vec<usize>,
        failed: Vec<(usize, Box<ClientError>)>,
    },

    #[error("Already authenticated")]
    AlreadyAuthenticated,
    #[error("Authentication required")]
    NotLoggedIn,
    #[error("Failed to find cache directory")]
    CacheDir,
    #[error("No values found in exercise details map returned by server")]
    MissingDetailsValue,
    #[error("List of exercises given was empty")]
    NoExercisesGiven,

    #[error(transparent)]
    SystemTime(#[from] std::time::SystemTimeError),
    #[error(transparent)]
    WalkDir(#[from] walkdir::Error),
    #[error("File IO error")]
    FileError(#[from] FileError),
    #[error(transparent)]
    Plugin(#[from] tmc_langs_plugins::PluginError),
}

impl From<TokenError> for ClientError {
    fn from(err: TokenError) -> Self {
        Self::Token(Box::new(err))
    }
}
