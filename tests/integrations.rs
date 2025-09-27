// tests/integrations.rs
use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::NamedTempFile;

/// Helper to read a test file
fn read_json(path: &std::path::Path) -> String {
    fs::read_to_string(path).expect("failed to read test file")
}

#[test]
fn add_and_list() {
    let tmp = NamedTempFile::new().expect("create temp file");
    let path = tmp.path().to_str().unwrap();

    // Add a task
    Command::cargo_bin("todo-cli")
        .unwrap()
        .args(["--file", path, "add", "Buy milk", "--notes", "from store"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Task added with id"));

    // The file should now contain a JSON array with at least one object
    let json = read_json(tmp.path());
    assert!(json.trim().starts_with('['), "todos file not an array");

    // List should show the task title
    Command::cargo_bin("todo-cli")
        .unwrap()
        .args(["--file", path, "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Buy milk"));
}

#[test]
fn mark_done_and_filtering() {
    let tmp = NamedTempFile::new().expect("create temp file");
    let path = tmp.path().to_str().unwrap();

    // Add two tasks
    Command::cargo_bin("todo-cli")
        .unwrap()
        .args(["--file", path, "add", "Task one"])
        .assert()
        .success();
    Command::cargo_bin("todo-cli")
        .unwrap()
        .args(["--file", path, "add", "Task two"])
        .assert()
        .success();

    // Mark the first task done (id 1)
    Command::cargo_bin("todo-cli")
        .unwrap()
        .args(["--file", path, "done", "1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Marked 1 done"));

    // Default list should not show completed tasks
    Command::cargo_bin("todo-cli")
        .unwrap()
        .args(["--file", path, "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Task two"))
        .stdout(predicate::str::contains("Task one").not());

    // list --all should show both
    Command::cargo_bin("todo-cli")
        .unwrap()
        .args(["--file", path, "list", "--all"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Task one"))
        .stdout(predicate::str::contains("Task two"));
}

#[test]
fn remove_task() {
    let tmp = NamedTempFile::new().expect("create temp file");
    let path = tmp.path().to_str().unwrap();

    // Add a task and remove it
    Command::cargo_bin("todo-cli")
        .unwrap()
        .args(["--file", path, "add", "To remove"])
        .assert()
        .success();

    // Remove id 1
    Command::cargo_bin("todo-cli")
        .unwrap()
        .args(["--file", path, "remove", "1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Removed 1"));

    // Listing should not show the removed task
    Command::cargo_bin("todo-cli")
        .unwrap()
        .args(["--file", path, "list", "--all"])
        .assert()
        .success()
        .stdout(predicate::str::contains("To remove").not());
}

// Optional: small unit-style tests verifying store::load behavior.
// If you want these to live here, import the function explicitly:
#[cfg(test)]
mod store_tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;
    use todo_cli::store::load; // <<-- import the function used below

    #[test]
    fn load_empty_file_returns_empty_vec() {
        let tmp = NamedTempFile::new().expect("create tmp file");
        // file is empty
        let p = tmp.path();
        let tasks = load(p).expect("load should succeed");
        assert!(tasks.is_empty());
    }

    #[test]
    fn load_empty_array_parses_ok() {
        let mut tmp = NamedTempFile::new().expect("create tmp file");
        write!(tmp, "[]").expect("write");
        let tasks = load(tmp.path()).expect("load should succeed");
        assert!(tasks.is_empty());
    }
}
