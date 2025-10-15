# gh-cli-rs

A Rust wrapper for the GitHub CLI (`gh`), providing a type-safe interface to all `gh` subcommands.

## Overview

This library provides a Rust interface to interact with the GitHub CLI tool, allowing you to programmatically execute `gh` commands with type safety and ergonomic APIs.

## Features

- Type-safe wrappers for all `gh` CLI subcommands
- Async/sync support
- Error handling with detailed error types
- Builder patterns for command construction

## Requirements

- GitHub CLI (`gh`) must be installed and available in your PATH
- Rust 1.70 or later

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
gh-cli-rs = "0.1.0"
```

## Usage

```rust
use gh_cli_rs::Gh;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example usage will be added as the library develops
    Ok(())
}
```

## License

MIT OR Apache-2.0
