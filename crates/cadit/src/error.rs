use std::{ffi::OsString, path::PathBuf};

use thiserror::Error;
use three_d::RendererError;

pub type CaditResult<T> = Result<T, CaditError>;

#[derive(Debug, Error)]
pub enum CaditError {
    #[error("Missing file extension")]
    MissingFileExtension,

    #[error("Invalid file extension `{0}`")]
    InvalidFileExtension(String),

    #[error("Invalid file extension `{}`", .0.to_string_lossy())]
    UnreadableFileExtension(OsString),

    #[error("Cannot open `{}` as a file because it is a directory", .0.to_string_lossy())]
    AttemptToOpenDirectoryAsFile(PathBuf),

    #[error("Renderer error: {0}")]
    RendererError(#[from] RendererError),
}
