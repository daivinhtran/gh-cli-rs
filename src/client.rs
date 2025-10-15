use crate::commands::{issue::IssueCommands, pr::PrCommands, repo::RepoCommands};
use crate::error::Result;
use crate::executor::GhExecutor;
use std::sync::Arc;

/// Main GitHub CLI client
/// This is the entry point for all GitHub CLI operations
#[derive(Clone)]
pub struct GhClient {
    executor: Arc<GhExecutor>,
}

impl Default for GhClient {
    fn default() -> Self {
        Self::new()
    }
}

impl GhClient {
    /// Create a new GitHub CLI client with default settings
    pub fn new() -> Self {
        Self {
            executor: Arc::new(GhExecutor::default()),
        }
    }

    /// Start building a custom GitHub CLI client
    pub fn builder() -> GhClientBuilder {
        GhClientBuilder::new()
    }

    /// Check if GitHub CLI is installed
    pub fn check_installation(&self) -> Result<String> {
        self.executor.check_installation()
    }

    /// Access repository commands
    pub fn repo(&self) -> RepoCommands {
        RepoCommands::new(self.executor.clone())
    }

    /// Access pull request commands
    pub fn pr(&self) -> PrCommands {
        PrCommands::new(self.executor.clone())
    }

    /// Access issue commands
    pub fn issue(&self) -> IssueCommands {
        IssueCommands::new(self.executor.clone())
    }
}

/// Builder for GhClient using the Builder Pattern
/// This allows for flexible configuration
pub struct GhClientBuilder {
    gh_path: Option<String>,
}

impl GhClientBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self { gh_path: None }
    }

    /// Set a custom path to the gh binary
    pub fn gh_path(mut self, path: impl Into<String>) -> Self {
        self.gh_path = Some(path.into());
        self
    }

    /// Build the GhClient
    pub fn build(self) -> GhClient {
        let executor = if let Some(gh_path) = self.gh_path {
            Arc::new(GhExecutor::new(gh_path))
        } else {
            Arc::new(GhExecutor::default())
        };

        GhClient { executor }
    }
}

impl Default for GhClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = GhClient::new();
        assert!(Arc::strong_count(&client.executor) >= 1);
    }

    #[test]
    fn test_builder_pattern() {
        let client = GhClient::builder().gh_path("/usr/local/bin/gh").build();
        assert!(Arc::strong_count(&client.executor) >= 1);
    }
}
