use super::*;
use std::fs::File;
use std::io::Write;
use tempfile::tempdir;
use tokio::runtime::Runtime;

#[test]
fn test_read_file_content() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test_file.rs");
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, "println!(\"Hello, Rust!\");").unwrap();

    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let content = read_file_content(&file_path).await.unwrap();
        assert_eq!(content, "println!(\"Hello, Rust!\");\n");
    });
}
