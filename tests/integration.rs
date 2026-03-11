use std::fs;
use std::process::Command;
use tempfile::TempDir;

fn cdi_bin() -> Command {
    let bin = env!("CARGO_BIN_EXE_cdi");
    Command::new(bin)
}

fn setup() -> TempDir {
    let tmp = TempDir::new().unwrap();
    let base = tmp.path();
    fs::create_dir(base.join("alpha")).unwrap();
    fs::create_dir(base.join("beta")).unwrap();
    fs::create_dir(base.join(".hidden")).unwrap();
    fs::File::create(base.join("file.txt")).unwrap();
    tmp
}

#[test]
fn test_list_output() {
    let tmp = setup();
    let output = cdi_bin().current_dir(tmp.path()).output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("0)"));
    assert!(stdout.contains("1)"));
    assert!(stdout.contains("2)"));
}

#[test]
fn test_navigate_valid_index() {
    let tmp = setup();
    let output = cdi_bin()
        .current_dir(tmp.path())
        .arg("1")
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    assert!(!stdout.is_empty());
    assert!(std::path::Path::new(&stdout).is_dir());
}

#[test]
fn test_navigate_out_of_range() {
    let tmp = setup();
    let output = cdi_bin()
        .current_dir(tmp.path())
        .arg("999")
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert_eq!(output.status.code(), Some(1));
}

#[test]
fn test_navigate_zero() {
    let tmp = setup();
    let output = cdi_bin()
        .current_dir(tmp.path())
        .arg("0")
        .output()
        .unwrap();
    assert!(output.status.success()); // 0은 유효한 인덱스
}

#[test]
fn test_navigate_invalid_string() {
    let tmp = setup();
    let output = cdi_bin()
        .current_dir(tmp.path())
        .arg("abc")
        .output()
        .unwrap();
    assert!(!output.status.success());
}

#[test]
fn test_version_flag() {
    let output = cdi_bin().arg("--version").output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.starts_with("cdi "));
}

#[test]
fn test_help_flag() {
    let output = cdi_bin().arg("--help").output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Usage"));
}

#[test]
fn test_too_many_args() {
    let tmp = setup();
    let output = cdi_bin()
        .current_dir(tmp.path())
        .args(["1", "2"])
        .output()
        .unwrap();
    assert!(!output.status.success());
}
