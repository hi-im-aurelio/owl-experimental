use std::path::Path;
use std::process::Command;

pub fn initialize_repo(clone_path: &str) {
    let repo_path = Path::new(clone_path);

    Command::new("git")
        .arg("init")
        .current_dir(repo_path)
        .status()
        .expect("Failed to initialize Git repository");

    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg("initial commit")
        .arg("--allow-empty")
        .current_dir(repo_path)
        .status()
        .expect("Failed to commit");

    Command::new("gh")
        .arg("repo")
        .arg("create")
        .arg(repo_path.file_name().unwrap().to_str().unwrap())
        .arg("--private")
        .current_dir(repo_path)
        .status()
        .expect("Failed to create GitHub repository");

    println!("Git repository initialized and pushed to GitHub successfully!");
}
