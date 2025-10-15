use crate::command::{BaseCommand, CommandBuilder, GhCommand};
use crate::error::Result;
use crate::executor::GhExecutor;
use std::sync::Arc;

/// Pull request commands namespace
#[derive(Clone)]
pub struct PrCommands {
    executor: Arc<GhExecutor>,
}

impl PrCommands {
    pub(crate) fn new(executor: Arc<GhExecutor>) -> Self {
        Self { executor }
    }

    /// Create a new pull request
    pub fn create(&self) -> PrCreateCommand {
        PrCreateCommand::new(self.executor.clone())
    }

    /// List pull requests
    pub fn list(&self) -> PrListCommand {
        PrListCommand::new(self.executor.clone())
    }

    /// View a pull request
    pub fn view(&self, number: u32) -> PrViewCommand {
        PrViewCommand::new(self.executor.clone(), number)
    }

    /// Checkout a pull request
    pub fn checkout(&self, number: u32) -> PrCheckoutCommand {
        PrCheckoutCommand::new(self.executor.clone(), number)
    }

    /// Merge a pull request
    pub fn merge(&self, number: u32) -> PrMergeCommand {
        PrMergeCommand::new(self.executor.clone(), number)
    }

    /// Close a pull request
    pub fn close(&self, number: u32) -> PrCloseCommand {
        PrCloseCommand::new(self.executor.clone(), number)
    }
}

/// Command for creating a pull request
pub struct PrCreateCommand {
    executor: Arc<GhExecutor>,
    cmd: BaseCommand,
}

impl PrCreateCommand {
    fn new(executor: Arc<GhExecutor>) -> Self {
        Self {
            executor,
            cmd: BaseCommand::with_subcommands(&["pr", "create"]),
        }
    }

    /// Set the PR title
    pub fn title(mut self, title: &str) -> Self {
        self.cmd = self.cmd.option("--title", title);
        self
    }

    /// Set the PR body
    pub fn body(mut self, body: &str) -> Self {
        self.cmd = self.cmd.option("--body", body);
        self
    }

    /// Set the base branch
    pub fn base(mut self, base: &str) -> Self {
        self.cmd = self.cmd.option("--base", base);
        self
    }

    /// Set the head branch
    pub fn head(mut self, head: &str) -> Self {
        self.cmd = self.cmd.option("--head", head);
        self
    }

    /// Mark as draft
    pub fn draft(mut self) -> Self {
        self.cmd = self.cmd.flag("--draft");
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

impl GhCommand for PrCreateCommand {
    fn build_args(&self) -> Vec<String> {
        self.cmd.build_args()
    }
}

/// Command for listing pull requests
pub struct PrListCommand {
    executor: Arc<GhExecutor>,
    cmd: BaseCommand,
}

impl PrListCommand {
    fn new(executor: Arc<GhExecutor>) -> Self {
        Self {
            executor,
            cmd: BaseCommand::with_subcommands(&["pr", "list"]),
        }
    }

    /// Filter by state (open, closed, merged, all)
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

    /// Execute and get raw output
    pub fn execute(&self) -> Result<String> {
        GhCommand::execute(self, self.executor.as_ref())
    }
}

impl GhCommand for PrListCommand {
    fn build_args(&self) -> Vec<String> {
        self.cmd.build_args()
    }
}

/// Command for viewing a pull request
pub struct PrViewCommand {
    executor: Arc<GhExecutor>,
    cmd: BaseCommand,
}

impl PrViewCommand {
    fn new(executor: Arc<GhExecutor>, number: u32) -> Self {
        Self {
            executor,
            cmd: BaseCommand::with_subcommands(&["pr", "view"]).arg(&number.to_string()),
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

impl GhCommand for PrViewCommand {
    fn build_args(&self) -> Vec<String> {
        self.cmd.build_args()
    }
}

/// Command for checking out a pull request
pub struct PrCheckoutCommand {
    executor: Arc<GhExecutor>,
    cmd: BaseCommand,
}

impl PrCheckoutCommand {
    fn new(executor: Arc<GhExecutor>, number: u32) -> Self {
        Self {
            executor,
            cmd: BaseCommand::with_subcommands(&["pr", "checkout"]).arg(&number.to_string()),
        }
    }

    /// Execute the checkout command
    pub fn execute(&self) -> Result<String> {
        GhCommand::execute(self, self.executor.as_ref())
    }
}

impl GhCommand for PrCheckoutCommand {
    fn build_args(&self) -> Vec<String> {
        self.cmd.build_args()
    }
}

/// Command for merging a pull request
pub struct PrMergeCommand {
    executor: Arc<GhExecutor>,
    cmd: BaseCommand,
}

impl PrMergeCommand {
    fn new(executor: Arc<GhExecutor>, number: u32) -> Self {
        Self {
            executor,
            cmd: BaseCommand::with_subcommands(&["pr", "merge"]).arg(&number.to_string()),
        }
    }

    /// Use merge commit
    pub fn merge(mut self) -> Self {
        self.cmd = self.cmd.flag("--merge");
        self
    }

    /// Use squash merge
    pub fn squash(mut self) -> Self {
        self.cmd = self.cmd.flag("--squash");
        self
    }

    /// Use rebase merge
    pub fn rebase(mut self) -> Self {
        self.cmd = self.cmd.flag("--rebase");
        self
    }

    /// Auto-merge when requirements are met
    pub fn auto(mut self) -> Self {
        self.cmd = self.cmd.flag("--auto");
        self
    }

    /// Execute the merge command
    pub fn execute(&self) -> Result<String> {
        GhCommand::execute(self, self.executor.as_ref())
    }
}

impl GhCommand for PrMergeCommand {
    fn build_args(&self) -> Vec<String> {
        self.cmd.build_args()
    }
}

/// Command for closing a pull request
pub struct PrCloseCommand {
    executor: Arc<GhExecutor>,
    cmd: BaseCommand,
}

impl PrCloseCommand {
    fn new(executor: Arc<GhExecutor>, number: u32) -> Self {
        Self {
            executor,
            cmd: BaseCommand::with_subcommands(&["pr", "close"]).arg(&number.to_string()),
        }
    }

    /// Delete the branch after closing
    pub fn delete_branch(mut self) -> Self {
        self.cmd = self.cmd.flag("--delete-branch");
        self
    }

    /// Execute the close command
    pub fn execute(&self) -> Result<String> {
        GhCommand::execute(self, self.executor.as_ref())
    }
}

impl GhCommand for PrCloseCommand {
    fn build_args(&self) -> Vec<String> {
        self.cmd.build_args()
    }
}
