use gh_cli_rs::GhClient;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = GhClient::new();

    println!("Creating a pull request...");
    match client
        .pr()
        .create()
        .title("test PR created by gh_cli_rs")
        .body("This PR adds a new feature\n\n## Changes\n- Added feature X\n- Fixed bug Y")
        .base("main")
        .head("feature-branch")
        .draft()
        .execute()
    {
        Ok(result) => println!("✓ PR created: {}", result),
        Err(e) => eprintln!("✗ Error: {}", e),
    }

    println!("\n🔍 Viewing PR #1:");
    match client.pr().view(1).execute() {
        Ok(pr) => println!("{}", pr),
        Err(e) => eprintln!("✗ Error: {}", e),
    }

    Ok(())
}
