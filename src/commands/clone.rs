use std::fs;
use std::io;
use std::path::Path;
use utils;

pub fn clone(source: &Path, destination: &Path, ignore_patterns: &[String]) -> io::Result<()> {
    if source.is_dir() {
        fs::create_dir_all(destination)?;
        for entry in fs::read_dir(source)? {
            let entry = entry?;
            let file_name = entry.file_name();
            let source_path = entry.path();
            let destination_path = destination.join(file_name);

            if utils::should_ignore::should_ignore(&source_path, ignore_patterns) {
                continue;
            }

            clone(&source_path, &destination_path, ignore_patterns)?;
        }
    } else if source.is_file() {
        fs::copy(source, destination)?;
        utils::permissions::set_read_only_permissions(destination)?;
    }

    Ok(())
}
