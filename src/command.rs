use crate::error::Result;
use crate::executor::GhExecutor;

/// Base trait for all GitHub CLI command builders
/// This implements the Command Pattern
pub trait GhCommand {
    /// Build the command arguments
    fn build_args(&self) -> Vec<String>;

    /// Execute the command and return raw string output
    fn execute(&self, executor: &GhExecutor) -> Result<String> {
        let args = self.build_args();
        executor.execute(&args)
    }
}

/// Helper trait for building commands with a fluent interface
pub trait CommandBuilder: Sized {
    /// Add a flag (e.g., "--web")
    fn flag(self, flag: &str) -> Self;

    /// Add a flag with a value (e.g., "--repo", "owner/name")
    fn option(self, key: &str, value: &str) -> Self;
}

/// Base command builder with common functionality
#[derive(Debug, Clone)]
pub struct BaseCommand {
    pub(crate) args: Vec<String>,
}

impl BaseCommand {
    /// Create a new base command with the given subcommand
    pub fn new(subcommand: &str) -> Self {
        Self {
            args: vec![subcommand.to_string()],
        }
    }

    /// Add multiple subcommands (e.g., ["pr", "list"])
    pub fn with_subcommands(subcommands: &[&str]) -> Self {
        Self {
            args: subcommands.iter().map(|s| s.to_string()).collect(),
        }
    }

    /// Add an argument
    pub fn arg(mut self, arg: &str) -> Self {
        self.args.push(arg.to_string());
        self
    }

    /// Add multiple arguments
    pub fn args(mut self, args: &[&str]) -> Self {
        self.args.extend(args.iter().map(|s| s.to_string()));
        self
    }
}

impl CommandBuilder for BaseCommand {
    fn flag(mut self, flag: &str) -> Self {
        self.args.push(flag.to_string());
        self
    }

    fn option(mut self, key: &str, value: &str) -> Self {
        self.args.push(key.to_string());
        self.args.push(value.to_string());
        self
    }
}

impl GhCommand for BaseCommand {
    fn build_args(&self) -> Vec<String> {
        self.args.clone()
    }
}
