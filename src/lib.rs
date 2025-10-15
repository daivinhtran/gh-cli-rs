//! # gh-cli-rs
//!
//! A Rust wrapper for the GitHub CLI (`gh`) with a fluent, type-safe API.
//!
//! This library provides a comprehensive interface to interact with GitHub CLI commands
//! using idiomatic Rust patterns including the Builder pattern, Command pattern, and
//! Fluent interfaces.
//!
//! ## Design Patterns
//!
//! - **Builder Pattern**: Used for configuring the `GhClient` with custom settings
//! - **Command Pattern**: Each CLI command is encapsulated in its own type
//! - **Fluent Interface**: Method chaining for ergonomic API usage
//! - **Strategy Pattern**: Pluggable command executors for testing and customization
//!
//! ## Examples
//!
//! ```no_run
//! use gh_cli_rs::GhClient;
//!
//! // Create a client
//! let client = GhClient::new();
//!
//! // List pull requests
//! let prs = client.pr()
//!     .list()
//!     .state("open")
//!     .limit(10)
//!     .execute();
//!
//! // Create an issue
//! let issue = client.issue()
//!     .create()
//!     .title("Bug report")
//!     .body("Something is broken")
//!     .label("bug")
//!     .execute();
//! ```

mod client;
mod command;
mod commands;
mod error;
mod executor;

// Public API exports
pub use client::{GhClient, GhClientBuilder};
pub use command::{BaseCommand, CommandBuilder, GhCommand};
pub use commands::*;
pub use error::{GhError, Result};
pub use executor::GhExecutor;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = GhClient::new();
        // Basic smoke test
        assert!(std::mem::size_of_val(&client) > 0);
    }

    #[test]
    fn test_builder_pattern() {
        let client = GhClient::builder().gh_path("/usr/local/bin/gh").build();
        assert!(std::mem::size_of_val(&client) > 0);
    }
}
