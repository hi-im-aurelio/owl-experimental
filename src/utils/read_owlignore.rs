use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

static OWLIGNORE_PATH: &str = "./src/.owlignore"; // to test

pub fn read_owlignore() -> io::Result<Vec<String>> {
    let mut ignore_patterns = Vec::new();

    if Path::new(OWLIGNORE_PATH).exists() {
        let file = fs::File::open(OWLIGNORE_PATH)?;
        let reader = io::BufReader::new(file);

        // Lê linha por linha
        for line in reader.lines() {
            let line = line?;
            if !line.trim().is_empty() {
                ignore_patterns.push(line);
            }
        }
    } else {
        panic!("No .owlignore file found at path {}", OWLIGNORE_PATH);
    }

    Ok(ignore_patterns)
}
