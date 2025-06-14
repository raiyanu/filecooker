use assert_cmd::Command;
use predicates::str::contains;
use std::fs;
use tempfile::tempdir;

#[test]
fn creates_file_and_directory() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("nested/dir/test.txt");

    let mut cmd = Command::cargo_bin("filecooker").unwrap();
    cmd.arg("create")
        .arg(file_path.to_str().unwrap())
        .env("FORCE_YN", "y") // Simulate 'yes'
        .assert()
        .success()
        .stdout(contains("Created:"));

    assert!(file_path.exists());
}

#[test]
fn skips_file_on_no_overwrite() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("existing.txt");

    // Pre-create file
    fs::write(&file_path, b"initial content").unwrap();

    let mut cmd = Command::cargo_bin("filecooker").unwrap();
    cmd.arg("create")
        .arg(file_path.to_str().unwrap())
        .env("FORCE_YN", "n") // Simulate 'no'
        .assert()
        .success()
        .stdout(contains("Skipped."));

    let content = fs::read(&file_path).unwrap();
    assert_eq!(content, b"initial content");
}
