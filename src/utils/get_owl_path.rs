use std::env;
use std::path::{Path, PathBuf};

pub fn owl_path() -> PathBuf {
    let home_dir = env::var("HOME").expect("Failed to get HOME directory");

    let path = Path::new(&home_dir).join(".owl");

    path
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_owl_path() {
        let r = owl_path();

        let home_dir = env::var("HOME").expect("Failed to get HOME directory");

        assert!(r.starts_with(&home_dir));
        assert!(r.ends_with(".owl"));

        println!("{:-^40}", "YOUR TEST");
        println!("owl path is: {}", r.display());
        println!("{}", "=".repeat(40));
    }
}
