use git2::Repository;
use std::path::PathBuf;

pub fn add_remote_origin_if_not_exists(
    repo_path: &PathBuf,
    remote_url: &str,
) -> Result<(), git2::Error> {
    let repo = Repository::open(repo_path)?;

    if repo.find_remote("origin").is_err() {
        println!("Remote 'origin' not found. Adding...");

        repo.remote("origin", remote_url)?;

        println!("Remote 'origin' added successfully.");
    } else {
        println!("Remote 'origin' already exists.");
    }

    Ok(())
}
