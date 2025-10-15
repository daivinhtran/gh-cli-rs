use gh_cli_rs::GhClient;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new GitHub CLI client
    let client = GhClient::new();

    // Check if gh CLI is installed
    match client.check_installation() {
        Ok(version) => println!("✓ GitHub CLI installed: {}", version.trim()),
        Err(e) => {
            eprintln!("✗ GitHub CLI not found: {}", e);
            eprintln!("Please install gh CLI from https://cli.github.com/");
            return Ok(());
        }
    }

    println!("📦 Listing your repositories (limit 5):");
    match client.repo().list().limit(5).execute() {
        Ok(repos) => println!("{}", repos),
        Err(e) => eprintln!("Error: {}", e),
    }

    println!("\n🔀 Listing open pull requests:");
    match client.pr().list().state("open").limit(5).execute() {
        Ok(prs) => println!("{}", prs),
        Err(e) => eprintln!("Error: {}", e),
    }

    println!("\n🐛 Listing open issues:");
    match client.issue().list().state("open").limit(5).execute() {
        Ok(issues) => println!("{}", issues),
        Err(e) => eprintln!("Error: {}", e),
    }

    println!("\n📊 Viewing repository details:");
    match client.repo().view(Some("daivinhtran/gh-cli-rs")).execute() {
        Ok(details) => println!("{}", details),
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}
