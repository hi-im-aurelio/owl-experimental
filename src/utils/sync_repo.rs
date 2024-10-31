use std::path::Path;
use std::process::Command;

use crate::commands;

use crate::utils;

pub fn sync_repo(original_repo: &Path, clone_repo: &Path) -> std::io::Result<()> {
    match utils::read_owlignore::read_owlignore() {
        Ok(patterns) => match commands::clone::clone(original_repo, clone_repo, &patterns) {
            Ok(_) => commands::git_add_commit_push::git_add_commit_push(clone_repo),
            Err(err) => {
                println!("Error in `sysnc_repo`: {}", err);
            }
        },
        Err(err) => {
            println!("Error sysnc repos: {}", err);
        }
    }

    Ok(())
}
