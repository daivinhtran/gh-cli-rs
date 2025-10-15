use gh_cli_rs::GhClient;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = GhClient::new();

    // Check if gh CLI is installed
    let version = client.check_installation()?;
    println!("✓ GitHub CLI: {}", version.trim());

    // List repositories
    let repos = client.repo().list().limit(5).execute()?;
    println!("{}", repos);

    Ok(())
}
