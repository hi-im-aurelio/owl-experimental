mod utils;
use utils::copy_files::copy_files;
use utils::read_owlignore::read_owlignore;

use std::io;
use std::path::Path;

fn main() -> io::Result<()> {
    let ignore_patterns = read_owlignore()?;
    let source_dir = Path::new("/home/ivy/srv/work/piminder/api/microservices/authentication");
    let destination_dir = Path::new("/home/ivy/.owl/clones/authentication");

    copy_files(source_dir, destination_dir, &ignore_patterns)?;

    println!("Arquivos copiados com sucesso!");

    Ok(())
}
