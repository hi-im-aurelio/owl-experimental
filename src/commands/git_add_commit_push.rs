use std::path::Path;
use std::process::Command;

pub fn git_add_commit_push(repo_path: &Path) {
    Command::new("git")
        .arg("add")
        .arg(".")
        .current_dir(repo_path)
        .status()
        .expect("Failed to add files to Git");

    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg("owl: automated commit")
        .current_dir(repo_path)
        .status()
        .expect("Failed to commit");

    Command::new("git")
        .arg("push")
        .arg("origin")
        .arg("master")
        .current_dir(repo_path)
        .status()
        .expect("Failed to push to GitHub");
}
