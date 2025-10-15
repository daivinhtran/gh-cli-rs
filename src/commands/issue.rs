use crate::command::{BaseCommand, CommandBuilder, GhCommand};
use crate::error::Result;
use crate::executor::GhExecutor;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Issue commands namespace
#[derive(Clone)]
pub struct IssueCommands {
    executor: Arc<GhExecutor>,
}

impl IssueCommands {
    pub(crate) fn new(executor: Arc<GhExecutor>) -> Self {
        Self { executor }
    }

    /// Create a new issue
    pub fn create(&self) -> IssueCreateCommand {
        IssueCreateCommand::new(self.executor.clone())
    }

    /// List issues
    pub fn list(&self) -> IssueListCommand {
        IssueListCommand::new(self.executor.clone())
    }

    /// View an issue
    pub fn view(&self, number: u32) -> IssueViewCommand {
        IssueViewCommand::new(self.executor.clone(), number)
    }

    /// Close an issue
    pub fn close(&self, number: u32) -> IssueCloseCommand {
        IssueCloseCommand::new(self.executor.clone(), number)
    }

    /// Reopen an issue
    pub fn reopen(&self, number: u32) -> IssueReopenCommand {
        IssueReopenCommand::new(self.executor.clone(), number)
    }
}

/// Issue information
#[derive(Debug, Serialize, Deserialize)]
pub struct Issue {
    pub number: u32,
    pub title: String,
    pub state: String,
    pub url: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    pub author: Author,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Author {
    pub login: String,
}

/// Command for creating an issue
pub struct IssueCreateCommand {
    executor: Arc<GhExecutor>,
    cmd: BaseCommand,
}

impl IssueCreateCommand {
    fn new(executor: Arc<GhExecutor>) -> Self {
        Self {
            executor,
            cmd: BaseCommand::with_subcommands(&["issue", "create"]),
        }
    }

    /// Set the issue title
    pub fn title(mut self, title: &str) -> Self {
        self.cmd = self.cmd.option("--title", title);
        self
    }

    /// Set the issue body
    pub fn body(mut self, body: &str) -> Self {
        self.cmd = self.cmd.option("--body", body);
        self
    }

    /// Add labels
    pub fn label(mut self, label: &str) -> Self {
        self.cmd = self.cmd.option("--label", label);
        self
    }

    /// Assign to user
    pub fn assignee(mut self, assignee: &str) -> Self {
        self.cmd = self.cmd.option("--assignee", assignee);
        self
    }

    /// Open in web browser
    pub fn web(mut self) -> Self {
        self.cmd = self.cmd.flag("--web");
        self
    }

    /// Execute the create command
    pub fn execute(&self) -> Result<String> {
        GhCommand::execute(self, self.executor.as_ref())
    }
}

impl GhCommand for IssueCreateCommand {
    fn build_args(&self) -> Vec<String> {
        self.cmd.build_args()
    }
}

/// Command for listing issues
pub struct IssueListCommand {
    executor: Arc<GhExecutor>,
    cmd: BaseCommand,
}

impl IssueListCommand {
    fn new(executor: Arc<GhExecutor>) -> Self {
        Self {
            executor,
            cmd: BaseCommand::with_subcommands(&["issue", "list"]),
        }
    }

    /// Filter by state (open, closed, all)
    pub fn state(mut self, state: &str) -> Self {
        self.cmd = self.cmd.option("--state", state);
        self
    }

    /// Limit the number of results
    pub fn limit(mut self, limit: u32) -> Self {
        self.cmd = self.cmd.option("--limit", &limit.to_string());
        self
    }

    /// Filter by author
    pub fn author(mut self, author: &str) -> Self {
        self.cmd = self.cmd.option("--author", author);
        self
    }

    /// Filter by assignee
    pub fn assignee(mut self, assignee: &str) -> Self {
        self.cmd = self.cmd.option("--assignee", assignee);
        self
    }

    /// Filter by label
    pub fn label(mut self, label: &str) -> Self {
        self.cmd = self.cmd.option("--label", label);
        self
    }

    /// Execute and get raw output
    pub fn execute(&self) -> Result<String> {
        GhCommand::execute(self, self.executor.as_ref())
    }
}

impl GhCommand for IssueListCommand {
    fn build_args(&self) -> Vec<String> {
        self.cmd.build_args()
    }
}

/// Command for viewing an issue
pub struct IssueViewCommand {
    executor: Arc<GhExecutor>,
    cmd: BaseCommand,
}

impl IssueViewCommand {
    fn new(executor: Arc<GhExecutor>, number: u32) -> Self {
        Self {
            executor,
            cmd: BaseCommand::with_subcommands(&["issue", "view"]).arg(&number.to_string()),
        }
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

impl GhCommand for IssueViewCommand {
    fn build_args(&self) -> Vec<String> {
        self.cmd.build_args()
    }
}

/// Command for closing an issue
pub struct IssueCloseCommand {
    executor: Arc<GhExecutor>,
    cmd: BaseCommand,
}

impl IssueCloseCommand {
    fn new(executor: Arc<GhExecutor>, number: u32) -> Self {
        Self {
            executor,
            cmd: BaseCommand::with_subcommands(&["issue", "close"]).arg(&number.to_string()),
        }
    }

    /// Execute the close command
    pub fn execute(&self) -> Result<String> {
        GhCommand::execute(self, self.executor.as_ref())
    }
}

impl GhCommand for IssueCloseCommand {
    fn build_args(&self) -> Vec<String> {
        self.cmd.build_args()
    }
}

/// Command for reopening an issue
pub struct IssueReopenCommand {
    executor: Arc<GhExecutor>,
    cmd: BaseCommand,
}

impl IssueReopenCommand {
    fn new(executor: Arc<GhExecutor>, number: u32) -> Self {
        Self {
            executor,
            cmd: BaseCommand::with_subcommands(&["issue", "reopen"]).arg(&number.to_string()),
        }
    }

    /// Execute the reopen command
    pub fn execute(&self) -> Result<String> {
        GhCommand::execute(self, self.executor.as_ref())
    }
}

impl GhCommand for IssueReopenCommand {
    fn build_args(&self) -> Vec<String> {
        self.cmd.build_args()
    }
}
