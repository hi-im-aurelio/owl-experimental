use std::io::{self, Write};
use std::path::PathBuf;

pub fn select_clone(clones: &[PathBuf]) -> io::Result<PathBuf> {
    println!("Select the clone you want to configure the origin for: ");
    for (i, clone) in clones.iter().enumerate() {
        let name = clone.file_name().unwrap().to_string_lossy();
        println!("{}. {}", i + 1, name);
    }

    print!("Enter the number of your choice: ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let choice: usize = input.trim().parse().expect("Invalid input");
    if choice == 0 || choice > clones.len() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Invalid choice",
        ));
    }

    Ok(clones[choice - 1].clone())
}
