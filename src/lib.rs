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
