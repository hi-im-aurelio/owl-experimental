use std::fs;
use std::io;
use std::path::Path;

pub fn set_read_only_permissions(file_path: &Path) -> io::Result<()> {
    let mut perms = fs::metadata(file_path)?.permissions();
    perms.set_readonly(true);
    fs::set_permissions(file_path, perms)
}
