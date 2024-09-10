use git2::Repository;
use std::path::Path;
use std::process::Command;

pub fn add_remote_origin_if_not_exists(
    repo_path: &str,
    remote_url: &str,
) -> Result<(), git2::Error> {
    let repo = Repository::open(repo_path)?;

    if repo.find_remote("origin").is_err() {
        println!("Remote 'origin' not found. Adding...");

        let output = Command::new("git")
            .arg("remote")
            .arg("add")
            .arg("origin")
            .arg(remote_url)
            .current_dir(repo_path)
            .output()
            .expect("Failed to add remote origin");

        if output.status.success() {
            println!("Remote 'origin' added successfully");
        } else {
            eprintln!(
                "Error adding remote 'origin': {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
    } else {
        println!("Remote 'origin'already exists.");
    }

    Ok(())
}
