use crate::command::{BaseCommand, CommandBuilder, GhCommand};
use crate::error::Result;
use crate::executor::GhExecutor;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Repository commands namespace
#[derive(Clone)]
pub struct RepoCommands {
    executor: Arc<GhExecutor>,
}

impl RepoCommands {
    pub(crate) fn new(executor: Arc<GhExecutor>) -> Self {
        Self { executor }
    }

    /// Clone a repository
    /// # Example
    /// ```no_run
    /// # use gh_cli_rs::GhClient;
    /// let client = GhClient::new();
    /// client.repo().clone("cli/cli").execute();
    /// ```
    pub fn clone(&self, repo: &str) -> RepoCloneCommand {
        RepoCloneCommand::new(self.executor.clone(), repo)
    }

    /// Create a new repository
    pub fn create(&self, name: &str) -> RepoCreateCommand {
        RepoCreateCommand::new(self.executor.clone(), name)
    }

    /// Fork a repository
    pub fn fork(&self, repo: &str) -> RepoForkCommand {
        RepoForkCommand::new(self.executor.clone(), repo)
    }

    /// List repositories
    pub fn list(&self) -> RepoListCommand {
        RepoListCommand::new(self.executor.clone())
    }

    /// View repository details
    pub fn view(&self, repo: Option<&str>) -> RepoViewCommand {
        RepoViewCommand::new(self.executor.clone(), repo)
    }
}

/// Command for cloning a repository
pub struct RepoCloneCommand {
    executor: Arc<GhExecutor>,
    cmd: BaseCommand,
}

impl RepoCloneCommand {
    fn new(executor: Arc<GhExecutor>, repo: &str) -> Self {
        Self {
            executor,
            cmd: BaseCommand::with_subcommands(&["repo", "clone"]).arg(repo),
        }
    }

    /// Execute the clone command
    pub fn execute(&self) -> Result<String> {
        GhCommand::execute(self, self.executor.as_ref())
    }
}

impl GhCommand for RepoCloneCommand {
    fn build_args(&self) -> Vec<String> {
        self.cmd.build_args()
    }
}

/// Command for creating a repository
pub struct RepoCreateCommand {
    executor: Arc<GhExecutor>,
    cmd: BaseCommand,
}

impl RepoCreateCommand {
    fn new(executor: Arc<GhExecutor>, name: &str) -> Self {
        Self {
            executor,
            cmd: BaseCommand::with_subcommands(&["repo", "create"]).arg(name),
        }
    }

    /// Set repository visibility to public
    pub fn public(mut self) -> Self {
        self.cmd = self.cmd.flag("--public");
        self
    }

    /// Set repository visibility to private
    pub fn private(mut self) -> Self {
        self.cmd = self.cmd.flag("--private");
        self
    }

    /// Add a description
    pub fn description(mut self, desc: &str) -> Self {
        self.cmd = self.cmd.option("--description", desc);
        self
    }

    /// Add a homepage URL
    pub fn homepage(mut self, url: &str) -> Self {
        self.cmd = self.cmd.option("--homepage", url);
        self
    }

    /// Initialize with a README
    pub fn with_readme(mut self) -> Self {
        self.cmd = self.cmd.flag("--add-readme");
        self
    }

    /// Execute the create command
    pub fn execute(&self) -> Result<String> {
        GhCommand::execute(self, self.executor.as_ref())
    }
}

impl GhCommand for RepoCreateCommand {
    fn build_args(&self) -> Vec<String> {
        self.cmd.build_args()
    }
}

/// Command for forking a repository
pub struct RepoForkCommand {
    executor: Arc<GhExecutor>,
    cmd: BaseCommand,
}

impl RepoForkCommand {
    fn new(executor: Arc<GhExecutor>, repo: &str) -> Self {
        Self {
            executor,
            cmd: BaseCommand::with_subcommands(&["repo", "fork"]).arg(repo),
        }
    }

    /// Clone the fork after creating it
    pub fn clone(mut self) -> Self {
        self.cmd = self.cmd.flag("--clone");
        self
    }

    /// Execute the fork command
    pub fn execute(&self) -> Result<String> {
        GhCommand::execute(self, self.executor.as_ref())
    }
}

impl GhCommand for RepoForkCommand {
    fn build_args(&self) -> Vec<String> {
        self.cmd.build_args()
    }
}

/// Repository information
#[derive(Debug, Serialize, Deserialize)]
pub struct Repository {
    pub name: String,
    #[serde(rename = "nameWithOwner")]
    pub name_with_owner: String,
    pub description: Option<String>,
    pub url: String,
    #[serde(rename = "isPrivate")]
    pub is_private: bool,
    #[serde(rename = "isFork")]
    pub is_fork: bool,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

/// Command for listing repositories
pub struct RepoListCommand {
    executor: Arc<GhExecutor>,
    cmd: BaseCommand,
}

impl RepoListCommand {
    fn new(executor: Arc<GhExecutor>) -> Self {
        Self {
            executor,
            cmd: BaseCommand::with_subcommands(&["repo", "list"]),
        }
    }

    /// List repositories for a specific owner
    pub fn owner(mut self, owner: &str) -> Self {
        self.cmd = self.cmd.arg(owner);
        self
    }

    /// Limit the number of results
    pub fn limit(mut self, limit: u32) -> Self {
        self.cmd = self.cmd.option("--limit", &limit.to_string());
        self
    }

    /// Execute and get raw output
    pub fn execute(&self) -> Result<String> {
        GhCommand::execute(self, self.executor.as_ref())
    }
}

impl GhCommand for RepoListCommand {
    fn build_args(&self) -> Vec<String> {
        self.cmd.build_args()
    }
}

/// Command for viewing repository details
pub struct RepoViewCommand {
    executor: Arc<GhExecutor>,
    cmd: BaseCommand,
}

impl RepoViewCommand {
    fn new(executor: Arc<GhExecutor>, repo: Option<&str>) -> Self {
        let mut cmd = BaseCommand::with_subcommands(&["repo", "view"]);
        if let Some(repo) = repo {
            cmd = cmd.arg(repo);
        }
        Self { executor, cmd }
    }

    /// Open in web browser
    pub fn web(mut self) -> Self {
        self.cmd = self.cmd.flag("--web");
        self
    }

    /// Execute and get raw output
    pub fn execute(&self) -> Result<String> {
        GhCommand::execute(self, self.executor.as_ref())
    }
}

impl GhCommand for RepoViewCommand {
    fn build_args(&self) -> Vec<String> {
        self.cmd.build_args()
    }
}
