# gh-cli-rs

A Rust wrapper for the GitHub CLI (`gh`), providing a type-safe, ergonomic interface to GitHub operations.

The library is implemented with a combination of [command pattern](https://en.wikipedia.org/wiki/Command_pattern), [fluent interface](https://en.wikipedia.org/wiki/Fluent_interface), and [builder pattern](https://en.wikipedia.org/wiki/Builder_pattern).

## Overview

`gh-cli-rs` is a Rust library that wraps the GitHub CLI tool, allowing you to programmatically execute `gh` commands with type safety and a fluent interface.

## Requirements

- **GitHub CLI (`gh`)** must be installed and authenticated
  ```bash
  # Install gh
  brew install gh  # macOS
  # or visit https://cli.github.com/
  
  # Authenticate
  gh auth login
  ```
- **Rust 1.56 or later**

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
gh-cli-rs = "0.1.0"
```

## Quick Start

```rust
use gh_cli_rs::GhClient;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client
    let client = GhClient::new();
    
    // Check installation
    let version = client.check_installation()?;
    println!("GitHub CLI version: {}", version);
    
    // List repositories
    let repos = client.repo().list().limit(5).execute()?;
    println!("{}", repos);
    
    Ok(())
}
```

## Usage Examples

### Repository Operations

```rust
use gh_cli_rs::GhClient;

let client = GhClient::new();

// List repositories
let repos = client.repo()
    .list()
    .limit(10)
    .execute()?;

// View repository details
let details = client.repo()
    .view(Some("owner/repo"))
    .execute()?;

// Create a repository
let result = client.repo()
    .create("my-new-repo")
    .public()
    .description("My awesome project")
    .execute()?;

// Clone a repository
let result = client.repo()
    .clone("owner/repo")
    .execute()?;
```

### Pull Request Operations

```rust
use gh_cli_rs::GhClient;

let client = GhClient::new();

// List pull requests
let prs = client.pr()
    .list()
    .state("open")
    .limit(10)
    .execute()?;

// Create a pull request (must be on a feature branch)
// Note: This requires you to have commits on your current branch
// that differ from the base branch
let pr = client.pr()
    .create()
    .title("Add new feature")
    .body("This PR adds an amazing feature")
    .base("main")
    .execute()?;

// View a pull request
let details = client.pr()
    .view(123)
    .execute()?;

// Checkout a pull request
client.pr().checkout(123).execute()?;

// Merge a pull request
client.pr()
    .merge(123)
    .squash()
    .execute()?;
```

**Important**: When creating a PR, ensure:
1. You're on a branch (not `main`)
2. Your branch has commits that differ from the base branch
3. You've pushed your branch to the remote

```bash
# Example workflow before creating PR
git checkout -b feature-branch
git add .
git commit -m "Add feature"
git push -u origin feature-branch

# Now you can create a PR via gh-cli-rs
```

### Issue Operations

```rust
use gh_cli_rs::GhClient;

let client = GhClient::new();

// List issues
let issues = client.issue()
    .list()
    .state("open")
    .limit(10)
    .execute()?;

// Create an issue
let issue = client.issue()
    .create()
    .title("Bug: Something is broken")
    .body("Detailed description of the bug")
    .execute()?;

// View an issue
let details = client.issue()
    .view(42)
    .execute()?;

// Close an issue
client.issue().close(42).execute()?;
```

### Authentication

```rust
use gh_cli_rs::GhClient;

let client = GhClient::new();

// Check authentication status
let status = client.auth().status().execute()?;

// Login (opens browser)
client.auth().login().execute()?;

// Logout
client.auth().logout().execute()?;
```

### Release Operations

```rust
use gh_cli_rs::GhClient;

let client = GhClient::new();

// List releases
let releases = client.release()
    .list()
    .limit(10)
    .execute()?;

// Create a release
let release = client.release()
    .create("v1.0.0")
    .title("Version 1.0.0")
    .notes("Release notes here")
    .execute()?;

// View a release
let details = client.release()
    .view("v1.0.0")
    .execute()?;
```

## Error Handling

The library provides detailed error types:

```rust
use gh_cli_rs::{GhClient, error::GhError};

let client = GhClient::new();

match client.repo().view(Some("nonexistent/repo")).execute() {
    Ok(details) => println!("{}", details),
    Err(GhError::GhNotFound) => {
        eprintln!("GitHub CLI not found. Please install from https://cli.github.com/");
    }
    Err(GhError::CommandFailed { exit_code, stderr }) => {
        eprintln!("Command failed with code {}: {}", exit_code, stderr);
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

## Design Patterns

This library uses several design patterns:

- **Builder Pattern**: Fluent API for constructing commands
- **Command Pattern**: Encapsulation of command execution
- **Fluent Interface**: Method chaining for readable code

## Custom Executor Path

If `gh` is not in your PATH:

```rust
use gh_cli_rs::GhClient;

let client = GhClient::builder()
    .gh_path("/custom/path/to/gh")
    .build();
```

## Examples

Check out the `examples/` directory:

```bash
cargo run --example basic_usage
cargo run --example pr
cargo run --example repo
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT OR Apache-2.0

## Acknowledgments

This library wraps the excellent [GitHub CLI](https://cli.github.com/) tool.
