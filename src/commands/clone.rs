use std::fs;
use std::io;
use std::path::Path;

use crate::utils;

pub fn clone(source: &Path, destination: &Path, ignore_patterns: &[String]) -> io::Result<()> {
    if source.is_dir() {
        // Cria o diretório de destino e todos os diretórios pai necessários
        if let Err(e) = fs::create_dir_all(destination) {
            eprintln!(
                "Failed to create directory '{}': {}",
                destination.display(),
                e
            );
            return Err(e);
        }

        for entry in fs::read_dir(source)? {
            let entry = entry?;
            let file_name = entry.file_name();
            let source_path = entry.path();
            let destination_path = destination.join(file_name);

            if utils::should_ignore::should_ignore(&source_path, ignore_patterns) {
                continue;
            }

            if let Err(e) = clone(&source_path, &destination_path, ignore_patterns) {
                eprintln!(
                    "Failed to clone from '{}' to '{}': {}",
                    source_path.display(),
                    destination_path.display(),
                    e
                );
                return Err(e);
            }
        }
    } else if source.is_file() {
        // Tenta copiar o arquivo
        if let Err(e) = fs::copy(source, destination) {
            eprintln!(
                "Failed to copy file from '{}' to '{}': {}",
                source.display(),
                destination.display(),
                e
            );
            return Err(e);
        }

        // Define permissões somente leitura
        if let Err(e) = utils::permissions::set_read_only_permissions(destination) {
            eprintln!(
                "Failed to set read-only permissions for '{}': {}",
                destination.display(),
                e
            );
            return Err(e);
        }
    }

    Ok(())
}
