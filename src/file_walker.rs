use std::path::Path;
use walkdir::{DirEntry, WalkDir};

/// Checks if a directory entry should be considered based on the given criteria.
fn should_include_entry(entry: &DirEntry, include_tests: bool, ext: &str) -> bool {
    let file_name = entry.file_name().to_string_lossy().to_lowercase();
    let is_test = file_name.contains("test");
    if !include_tests && is_test {
        return false;
    }

    entry.file_type().is_file() && entry.path().extension().map_or(false, |e| e == ext)
}

/// Walks through a directory and returns a list of files that match the criteria.
pub fn walk_directory(path: &Path, include_tests: bool, ext: &str) -> Vec<DirEntry> {
    WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| should_include_entry(e, include_tests, ext))
        .collect()
}

#[cfg(test)]
mod tests;
