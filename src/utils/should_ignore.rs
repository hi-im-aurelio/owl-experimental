use std::path::Path;

pub fn should_ignore(file_path: &Path, ignore_patterns: &[String]) -> bool {
    for pattern in ignore_patterns {
        if file_path.ends_with(pattern) {
            return true;
        }
    }
    false
}
