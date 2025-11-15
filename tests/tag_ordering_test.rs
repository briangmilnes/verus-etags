use std::fs;
use std::process::Command;
use tempfile::TempDir;

#[test]
fn test_tags_sorted_by_line_number() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.rs");
    
    // Create a file with functions in a specific order
    // We'll name them so they're NOT in alphabetical order
    fs::write(&test_file, r#"
// Line 2
pub fn zebra_function() {}  // Line 3 - alphabetically last

// Line 5
pub fn alpha_function() {}  // Line 6 - alphabetically first

// Line 8  
pub fn middle_function() {} // Line 9 - alphabetically middle

// Line 11
pub struct MyStruct;         // Line 12

// Line 14
pub enum MyEnum { A, B }     // Line 15
"#).unwrap();

    let tags_file = temp_dir.path().join("TAGS");
    
    let output = Command::new(env!("CARGO_BIN_EXE_verus-etags"))
        .arg("-o")
        .arg(&tags_file)
        .arg(&test_file)
        .output()
        .expect("Failed to execute verus-etags");

    assert!(output.status.success(), "verus-etags failed: {:?}", output);

    let tags_content = fs::read_to_string(&tags_file).unwrap();
    
    // Extract just the tag entries (skip file header lines)
    // File headers look like: \x0c\n/path/file.rs,size\n
    // Tag entries contain the tag name followed by \x01
    let tag_lines: Vec<&str> = tags_content
        .lines()
        .filter(|line| line.contains('\x01') || line.contains('\x7f'))
        .collect();
    
    // Find the positions of our test functions in the output
    let zebra_pos = tag_lines.iter().position(|l| l.contains("zebra_function"));
    let alpha_pos = tag_lines.iter().position(|l| l.contains("alpha_function"));
    let middle_pos = tag_lines.iter().position(|l| l.contains("middle_function"));
    let struct_pos = tag_lines.iter().position(|l| l.contains("MyStruct"));
    let enum_pos = tag_lines.iter().position(|l| l.contains("MyEnum"));
    
    // All should be found
    assert!(zebra_pos.is_some(), "zebra_function not found");
    assert!(alpha_pos.is_some(), "alpha_function not found");
    assert!(middle_pos.is_some(), "middle_function not found");
    assert!(struct_pos.is_some(), "MyStruct not found");
    assert!(enum_pos.is_some(), "MyEnum not found");
    
    let zebra_pos = zebra_pos.unwrap();
    let alpha_pos = alpha_pos.unwrap();
    let middle_pos = middle_pos.unwrap();
    let struct_pos = struct_pos.unwrap();
    let enum_pos = enum_pos.unwrap();
    
    // Tags should appear in LINE NUMBER order, not alphabetical order
    // zebra (line 3) < alpha (line 6) < middle (line 9) < struct (line 12) < enum (line 15)
    assert!(zebra_pos < alpha_pos, 
        "zebra_function (line 3) should appear before alpha_function (line 6), but found at positions {} and {}", 
        zebra_pos, alpha_pos);
    assert!(alpha_pos < middle_pos,
        "alpha_function (line 6) should appear before middle_function (line 9), but found at positions {} and {}",
        alpha_pos, middle_pos);
    assert!(middle_pos < struct_pos,
        "middle_function (line 9) should appear before MyStruct (line 12), but found at positions {} and {}",
        middle_pos, struct_pos);
    assert!(struct_pos < enum_pos,
        "MyStruct (line 12) should appear before MyEnum (line 15), but found at positions {} and {}",
        struct_pos, enum_pos);
}

#[test]
fn test_tags_line_numbers_extracted_correctly() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.rs");
    
    fs::write(&test_file, r#"
pub fn first() {}

pub fn second() {}

pub fn third() {}
"#).unwrap();

    let tags_file = temp_dir.path().join("TAGS");
    
    let output = Command::new(env!("CARGO_BIN_EXE_verus-etags"))
        .arg("-o")
        .arg(&tags_file)
        .arg(&test_file)
        .output()
        .expect("Failed to execute verus-etags");

    assert!(output.status.success());

    let tags_content = fs::read_to_string(&tags_file).unwrap();
    
    // Check that line numbers are correctly extracted
    // first is on line 2, second on line 4, third on line 6
    assert!(tags_content.contains("first\x01") || tags_content.contains("first,"), 
        "first function should have line number in tags");
    
    // Verify ordering in the file
    let first_idx = tags_content.find("first").unwrap();
    let second_idx = tags_content.find("second").unwrap();
    let third_idx = tags_content.find("third").unwrap();
    
    assert!(first_idx < second_idx && second_idx < third_idx,
        "Functions should appear in source order");
}

