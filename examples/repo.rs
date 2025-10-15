use gh_cli_rs::GhClient;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = GhClient::new();
    println!("Creating a new repository:");
    match client
        .repo()
        .create("gh-cli-rs")
        .description("A wrapper for GitHub CLI in Rust")
        .public()
        .with_readme()
        .execute()
    {
        Ok(result) => println!("✓ Repository created: {}", result),
        Err(e) => eprintln!("✗ Error: {}", e),
    }

    Ok(())
}
