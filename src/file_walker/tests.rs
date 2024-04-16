use super::*;
use std::fs::File;
use std::io::Write;
use tempfile::tempdir;

#[test]
fn test_walk_directory() {
    let dir = tempdir().unwrap();
    let test_file_path = dir.path().join("test_file.rs");
    let normal_file_path = dir.path().join("normal_file.rs");

    let mut test_file = File::create(&test_file_path).unwrap();
    writeln!(test_file, "println!(\"This is a test file.\");").unwrap();
    test_file.sync_all().unwrap(); // 确保写入完成

    let mut normal_file = File::create(&normal_file_path).unwrap();
    writeln!(normal_file, "println!(\"This is a normal file.\");").unwrap();
    normal_file.sync_all().unwrap(); // 确保写入完成

    // Include tests = false
    let files_without_tests = walk_directory(dir.path(), false, "rs");
    assert_eq!(files_without_tests.len(), 1);
    assert_eq!(files_without_tests[0].path(), normal_file_path);

    // Include tests = true
    let files_with_tests = walk_directory(dir.path(), true, "rs");
    assert_eq!(files_with_tests.len(), 2);
    assert!(files_with_tests
        .iter()
        .any(|entry| entry.path() == test_file_path));
    assert!(files_with_tests
        .iter()
        .any(|entry| entry.path() == normal_file_path));
}
