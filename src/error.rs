use std::process::Output;
use thiserror::Error;

/// Result type for gh-cli-rs operations
pub type Result<T> = std::result::Result<T, GhError>;

/// Error types that can occur when executing GitHub CLI commands
#[derive(Error, Debug)]
pub enum GhError {
    #[error("GitHub CLI is not installed or not found in PATH")]
    GhNotFound,

    #[error("Command execution failed: {0}")]
    ExecutionFailed(String),

    #[error("Command failed with exit code {code}: {stderr}")]
    CommandFailed { code: i32, stderr: String },

    #[error("Failed to parse JSON output: {0}")]
    JsonParseError(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("UTF-8 conversion error: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),

    #[error("Invalid command: {0}")]
    InvalidCommand(String),
}

impl GhError {
    /// Create a CommandFailed error from a process output
    pub fn from_output(output: Output) -> Self {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let code = output.status.code().unwrap_or(-1);
        GhError::CommandFailed { code, stderr }
    }
}
