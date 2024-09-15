use std::fs;
use std::io::{self};
use std::path::PathBuf;

use crate::utils;

pub fn list_clones() -> io::Result<Vec<PathBuf>> {
    let owl = utils::get_owl_path::owl_path();
    let clone_dir = owl.join("clones");

    let mut clones = Vec::new();

    for entry in fs::read_dir(clone_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            clones.push(path);
        }
    }

    Ok(clones)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_clones() {
        let _r = list_clones();
    }
}
