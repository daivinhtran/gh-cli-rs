use crate::error::{GhError, Result};
use std::process::{Command, Stdio};

/// Executor for GitHub CLI commands
#[derive(Debug, Clone)]
pub struct GhExecutor {
    /// Path to the gh binary (defaults to "gh")
    pub gh_path: String,
}

impl Default for GhExecutor {
    fn default() -> Self {
        Self {
            gh_path: "gh".to_string(),
        }
    }
}

impl GhExecutor {
    /// Create a new executor with a custom gh binary path
    pub fn new(gh_path: String) -> Self {
        Self { gh_path }
    }

    /// Check if gh CLI is installed and accessible
    pub fn check_installation(&self) -> Result<String> {
        let output = Command::new(&self.gh_path)
            .arg("--version")
            .output()
            .map_err(|_| GhError::GhNotFound)?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(GhError::GhNotFound)
        }
    }

    /// Execute a gh command with the given arguments
    pub fn execute(&self, args: &[String]) -> Result<String> {
        let output = Command::new(&self.gh_path)
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()?;

        if output.status.success() {
            Ok(String::from_utf8(output.stdout)?)
        } else {
            Err(GhError::from_output(output))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_executor_creation() {
        let executor = GhExecutor::default();
        assert_eq!(executor.gh_path, "gh");

        let custom_executor = GhExecutor::new("/custom/path/gh".to_string());
        assert_eq!(custom_executor.gh_path, "/custom/path/gh");
    }
}
