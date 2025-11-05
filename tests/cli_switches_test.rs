use std::fs;
use std::process::Command;

#[test]
fn test_version_short_flag() {
    let output = Command::new("./target/release/verus-etags")
        .arg("-v")
        .output()
        .expect("Failed to execute verus-etags");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("1.0.0"));
}

#[test]
fn test_version_flag() {
    let output = Command::new("./target/release/verus-etags")
        .arg("--version")
        .output()
        .expect("Failed to execute verus-etags");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("1.0.0"));
}

#[test]
fn test_help_flag() {
    let output = Command::new("./target/release/verus-etags")
        .arg("--help")
        .output()
        .expect("Failed to execute verus-etags");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Generate etags"));
}

#[test]
fn test_output_short_flag() {
    let output_file = "test_output_short.tags";
    let _ = fs::remove_file(output_file);

    let output = Command::new("./target/release/verus-etags")
        .args(&["-o", output_file, "test_data/simple.rs"])
        .output()
        .expect("Failed to execute verus-etags");

    assert!(output.status.success());
    assert!(std::path::Path::new(output_file).exists());
    
    fs::remove_file(output_file).ok();
}

#[test]
fn test_output_long_flag() {
    let output_file = "test_output_long.tags";
    let _ = fs::remove_file(output_file);

    let output = Command::new("./target/release/verus-etags")
        .args(&["--output", output_file, "test_data/simple.rs"])
        .output()
        .expect("Failed to execute verus-etags");

    assert!(output.status.success());
    assert!(std::path::Path::new(output_file).exists());
    
    fs::remove_file(output_file).ok();
}

#[test]
fn test_file_alias_flag() {
    let output_file = "test_file_alias.tags";
    let _ = fs::remove_file(output_file);

    let output = Command::new("./target/release/verus-etags")
        .args(&["--file", output_file, "test_data/simple.rs"])
        .output()
        .expect("Failed to execute verus-etags");

    assert!(output.status.success());
    assert!(std::path::Path::new(output_file).exists());
    
    fs::remove_file(output_file).ok();
}

#[test]
fn test_file_long_alias_flag() {
    let output_file = "test_file_long.tags";
    let _ = fs::remove_file(output_file);

    let output = Command::new("./target/release/verus-etags")
        .args(&["-f", output_file, "test_data/simple.rs"])
        .output()
        .expect("Failed to execute verus-etags");

    assert!(output.status.success());
    assert!(std::path::Path::new(output_file).exists());
    
    fs::remove_file(output_file).ok();
}

#[test]
fn test_default_output_file() {
    let output_file = "TAGS";
    let _ = fs::remove_file(output_file);

    let output = Command::new("./target/release/verus-etags")
        .arg("test_data/simple.rs")
        .output()
        .expect("Failed to execute verus-etags");

    assert!(output.status.success());
    assert!(std::path::Path::new(output_file).exists());
    
    fs::remove_file(output_file).ok();
}

#[test]
fn test_append_flag() {
    let output_file = "test_append.tags";
    let _ = fs::remove_file(output_file);

    // Create initial file
    Command::new("./target/release/verus-etags")
        .args(&["-o", output_file, "test_data/simple.rs"])
        .output()
        .expect("Failed to execute verus-etags");

    // Append to it
    let output = Command::new("./target/release/verus-etags")
        .args(&["-a", "-o", output_file, "test_data/another.rs"])
        .output()
        .expect("Failed to execute verus-etags");

    assert!(output.status.success());
    
    fs::remove_file(output_file).ok();
}

#[test]
fn test_verbose_short_flag() {
    let output_file = "test_verbose_short.tags";
    let _ = fs::remove_file(output_file);

    let output = Command::new("./target/release/verus-etags")
        .args(&["-V", "-o", output_file, "test_data/simple.rs"])
        .output()
        .expect("Failed to execute verus-etags");

    assert!(output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Processing file"));
    
    fs::remove_file(output_file).ok();
}

#[test]
fn test_verbose_long_flag() {
    let output_file = "test_verbose_long.tags";
    let _ = fs::remove_file(output_file);

    let output = Command::new("./target/release/verus-etags")
        .args(&["--verbose", "-o", output_file, "test_data/simple.rs"])
        .output()
        .expect("Failed to execute verus-etags");

    assert!(output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Processing file"));
    
    fs::remove_file(output_file).ok();
}

#[test]
fn test_sort_unsorted() {
    let output_file = "test_sort_unsorted.tags";
    let _ = fs::remove_file(output_file);

    let output = Command::new("./target/release/verus-etags")
        .args(&["-s", "0", "-o", output_file, "test_data/simple.rs"])
        .output()
        .expect("Failed to execute verus-etags");

    assert!(output.status.success());
    
    fs::remove_file(output_file).ok();
}

#[test]
fn test_sort_sorted() {
    let output_file = "test_sort_sorted.tags";
    let _ = fs::remove_file(output_file);

    let output = Command::new("./target/release/verus-etags")
        .args(&["-s", "1", "-o", output_file, "test_data/simple.rs"])
        .output()
        .expect("Failed to execute verus-etags");

    assert!(output.status.success());
    
    fs::remove_file(output_file).ok();
}

#[test]
fn test_sort_foldcase() {
    let output_file = "test_sort_foldcase.tags";
    let _ = fs::remove_file(output_file);

    let output = Command::new("./target/release/verus-etags")
        .args(&["-s", "2", "-o", output_file, "test_data/simple.rs"])
        .output()
        .expect("Failed to execute verus-etags");

    assert!(output.status.success());
    
    fs::remove_file(output_file).ok();
}

#[test]
fn test_recurse_flag() {
    let output_file = "test_recurse.tags";
    let _ = fs::remove_file(output_file);

    let output = Command::new("./target/release/verus-etags")
        .args(&["-R", "-o", output_file, "test_data"])
        .output()
        .expect("Failed to execute verus-etags");

    assert!(output.status.success());
    
    fs::remove_file(output_file).ok();
}

#[test]
fn test_recurse_long_flag() {
    let output_file = "test_recurse_long.tags";
    let _ = fs::remove_file(output_file);

    let output = Command::new("./target/release/verus-etags")
        .args(&["--recurse", "-o", output_file, "test_data"])
        .output()
        .expect("Failed to execute verus-etags");

    assert!(output.status.success());
    
    fs::remove_file(output_file).ok();
}

#[test]
fn test_no_recurse_flag() {
    let output_file = "test_no_recurse.tags";
    let _ = fs::remove_file(output_file);

    let output = Command::new("./target/release/verus-etags")
        .args(&["--no-recurse", "-o", output_file, "test_data"])
        .output()
        .expect("Failed to execute verus-etags");

    assert!(output.status.success());
    
    fs::remove_file(output_file).ok();
}

#[test]
fn test_default_recurse_behavior() {
    let output_file = "test_default_recurse.tags";
    let _ = fs::remove_file(output_file);

    let output = Command::new("./target/release/verus-etags")
        .args(&["-o", output_file, "test_data"])
        .output()
        .expect("Failed to execute verus-etags");

    assert!(output.status.success());
    
    fs::remove_file(output_file).ok();
}

#[test]
fn test_multiple_files() {
    let output_file = "test_multiple.tags";
    let _ = fs::remove_file(output_file);

    let output = Command::new("./target/release/verus-etags")
        .args(&["-o", output_file, "test_data/simple.rs", "test_data/another.rs"])
        .output()
        .expect("Failed to execute verus-etags");

    assert!(output.status.success());
    
    fs::remove_file(output_file).ok();
}

#[test]
fn test_directory_input() {
    let output_file = "test_directory.tags";
    let _ = fs::remove_file(output_file);

    let output = Command::new("./target/release/verus-etags")
        .args(&["-o", output_file, "test_data"])
        .output()
        .expect("Failed to execute verus-etags");

    assert!(output.status.success());
    
    fs::remove_file(output_file).ok();
}

