use std::path::PathBuf;

use thiserror::Error;
use tmc_client::ClientError;

use crate::course_refresher::ModeBits;
#[derive(Error, Debug)]
pub enum LangsError {
    #[error("Failed to create temporary file")]
    TempFile(#[source] std::io::Error),
    #[error("Failed to create temporary directory")]
    TempDir(#[source] std::io::Error),
    #[error("Invalid parameter key/value: {0}")]
    InvalidParam(String, #[source] ParamError),
    #[error("Error compressing file at {0} with zstd")]
    Zstd(PathBuf, #[source] std::io::Error),
    #[error("Error retrieving file handle from tar builder")]
    TarIntoInner(#[source] std::io::Error),
    #[error("Error finishing tar")]
    TarFinish(#[source] std::io::Error),
    #[error("Error appending path {0} to tar")]
    TarAppend(PathBuf, #[source] std::io::Error),
    #[error("Failed to aquire mutex")]
    MutexError,
    #[error("No project directory found in archive during unzip")]
    NoProjectDirInZip(PathBuf),
    #[error("Error while writing file to zip")]
    ZipWrite(#[source] std::io::Error),
    #[error("Failed to parse file {0}")]
    SubmissionParse(PathBuf, #[source] Box<Self>),
    #[error("Failed to deserialize credentials file at {0}. The file has been removed, please try again")]
    DeserializeCredentials(PathBuf, #[source] serde_json::Error),
    #[error("No local data directory found")]
    NoLocalDataDir,
    #[error("No config directory found")]
    NoConfigDir,
    #[error("Expected directory at {0} to be empty")]
    NonEmptyDir(PathBuf),
    #[error("Directory {0} already exists")]
    DirectoryExists(PathBuf),
    #[error("The value for projects-dir must be a string.")]
    ProjectsDirNotString,
    #[error("Attempted to move the projects-dir to the directory it's already in")]
    MovingProjectsDirToItself,
    #[error("No projects-dir found")]
    NoProjectsDir,

    #[error("Cache path {0} was invalid. Not a valid UTF-8 string or did not contain a cache version after a dash")]
    InvalidCachePath(PathBuf),
    #[error("Path {0} contained a dash '-' which is currently not allowed")]
    InvalidDirectory(PathBuf),

    #[cfg(unix)]
    #[error("Error changing permissions of {0}")]
    NixPermissionChange(PathBuf, #[source] nix::Error),
    #[cfg(unix)]
    #[error("Invalid chmod flag: {0}")]
    NixFlag(ModeBits),

    #[error("Failed to canonicalize path {0}")]
    Canonicalize(PathBuf, #[source] std::io::Error),

    #[error(transparent)]
    TmcClient(#[from] ClientError),
    #[error(transparent)]
    Tmc(#[from] tmc_langs_framework::TmcError),
    #[error(transparent)]
    Plugin(#[from] tmc_langs_plugins::PluginError),
    #[error(transparent)]
    FileError(#[from] tmc_langs_util::FileError),
    #[error(transparent)]
    Heim(#[from] heim::Error),
    #[error(transparent)]
    WalkDir(#[from] walkdir::Error),
    #[error(transparent)]
    Zip(#[from] zip::result::ZipError),
    #[error(transparent)]
    Yaml(#[from] serde_yaml::Error),
    #[error(transparent)]
    TomlSerialize(#[from] toml::ser::Error),
    #[error(transparent)]
    TomlDeserialize(#[from] toml::de::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

#[derive(Debug, Error)]
pub enum ParamError {
    #[error("Parameter key/value was empty")]
    Empty,
    #[error("Invalid character found in key/value: {0}")]
    InvalidChar(char),
}
