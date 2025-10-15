# gh-cli-rs

🦀 A Rust wrapper for the GitHub CLI with a type-safe, fluent API. Built with [Command Pattern](https://en.wikipedia.org/wiki/Command_pattern), [Fluent Interface](https://en.wikipedia.org/wiki/Fluent_interface), and [Builder Pattern](https://en.wikipedia.org/wiki/Builder_pattern).

There are tons of commands in https://cli.github.com/manual/gh not implemented yet. PRs welcome!

I plan to add support for all the core commands in the near future.

## Use Cases

- Build [gh extensions](https://github.com/topics/gh-extension) in Rust
- Automate GitHub workflows (CI/CD, release management)
- CLI tools with GitHub integration
- Scripts for repo/PR/issue management

## Requirements

- [GitHub CLI](https://cli.github.com/) installed and authenticated
- Rust 1.56+

```bash
brew install gh && gh auth login
```

## Installation

```toml
[dependencies]
gh-cli-rs = "0.1.0"
```

## Quick Start

```rust
use gh_cli_rs::GhClient;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = GhClient::new();
    
    let version = client.check_installation()?;
    println!("GitHub CLI: {}", version);
    
    let repos = client.repo().list().limit(5).execute()?;
    println!("{}", repos);
    
    Ok(())
}
```

## Usage

### Repositories

```rust
client.repo().list().limit(10).execute()?;
client.repo().view(Some("owner/repo")).execute()?;
client.repo().create("my-repo").public().description("Cool project").execute()?;
client.repo().clone("owner/repo").execute()?;
```

### Pull Requests

```rust
client.pr().list().state("open").limit(10).execute()?;
client.pr().create().title("Fix bug").body("Details").base("main").execute()?;
client.pr().view(123).execute()?;
client.pr().checkout(123).execute()?;
client.pr().merge(123).squash().execute()?;
```

**Note:** To create a PR, be on a feature branch with commits pushed to remote.

### Issues

```rust
client.issue().list().state("open").execute()?;
client.issue().create().title("Bug").body("Details").execute()?;
client.issue().view(42).execute()?;
client.issue().close(42).execute()?;
```

### Auth & Releases

```rust
// Auth
client.auth().status().execute()?;
client.auth().login().execute()?;
client.auth().logout().execute()?;

// Releases
client.release().list().limit(10).execute()?;
client.release().create("v1.0.0").title("Release").notes("...").execute()?;
client.release().view("v1.0.0").execute()?;
```

## Error Handling

```rust
use gh_cli_rs::error::GhError;

match client.repo().view(Some("bad/repo")).execute() {
    Ok(output) => println!("{}", output),
    Err(GhError::GhNotFound) => eprintln!("gh CLI not found"),
    Err(GhError::CommandFailed { stderr, .. }) => eprintln!("{}", stderr),
    Err(e) => eprintln!("{}", e),
}
```

## Custom gh Path

```rust
let client = GhClient::builder().gh_path("/custom/path/gh").build();
```

## Examples

```bash
cargo run --example basic_usage
```

## License

MIT OR Apache-2.0
