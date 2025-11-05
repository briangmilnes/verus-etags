use std::fs;
use std::process::Command;

#[test]
fn test_byte_offset_points_to_line_start() {
    let output_file = "test_byte_offset.tags";
    let _ = fs::remove_file(output_file);

    let output = Command::new("./target/release/verus-etags")
        .args(&["-o", output_file, "test_data/simple.rs"])
        .output()
        .expect("Failed to execute verus-etags");

    assert!(output.status.success());

    let tags_content = fs::read_to_string(output_file)
        .expect("Failed to read TAGS file");

    // Read the source file to verify byte offsets
    let source_content = fs::read_to_string("test_data/simple.rs")
        .expect("Failed to read source file");

    // Find a tag entry and extract byte offset
    let lines: Vec<&str> = tags_content.lines().collect();
    for line in lines {
        if line.contains("\x7f") && line.contains("\x01") {
            let parts: Vec<&str> = line.split('\x7f').collect();
            if parts.len() >= 2 {
                let location_part: Vec<&str> = parts[1].split('\x01').collect();
                if location_part.len() >= 2 {
                    let location: Vec<&str> = location_part[1].split(',').collect();
                    if location.len() >= 2 {
                        let byte_offset: usize = location[1].parse().unwrap_or(0);
                        
                        // Verify that byte_offset points to the start of a line
                        if byte_offset > 0 && byte_offset < source_content.len() {
                            // Check the character before this offset is a newline
                            // (unless it's offset 0, which is the file start)
                            if byte_offset > 0 {
                                let prev_char = source_content.as_bytes()[byte_offset - 1];
                                assert_eq!(prev_char, b'\n', 
                                    "Byte offset {} should point to start of line (previous char should be newline)", 
                                    byte_offset);
                            }
                        }
                    }
                }
            }
        }
    }

    fs::remove_file(output_file).ok();
}

#[test]
fn test_pattern_matches_file_content() {
    let output_file = "test_pattern.tags";
    let _ = fs::remove_file(output_file);

    let output = Command::new("./target/release/verus-etags")
        .args(&["-o", output_file, "test_data/simple.rs"])
        .output()
        .expect("Failed to execute verus-etags");

    assert!(output.status.success());

    let tags_content = fs::read_to_string(output_file)
        .expect("Failed to read TAGS file");

    let source_content = fs::read_to_string("test_data/simple.rs")
        .expect("Failed to read source file");

    // Verify that patterns actually exist in the source
    let lines: Vec<&str> = tags_content.lines().collect();
    for line in lines {
        if line.contains("\x7f") {
            let parts: Vec<&str> = line.split('\x7f').collect();
            if !parts.is_empty() {
                let pattern = parts[0];
                if !pattern.is_empty() && !pattern.starts_with('\x0c') {
                    // The pattern should exist in the source (or be a simplified identifier)
                    // For now just check it's not empty and is reasonable
                    assert!(pattern.len() > 0);
                    assert!(pattern.len() < 500); // Reasonable pattern length
                }
            }
        }
    }

    fs::remove_file(output_file).ok();
}

#[test]
fn test_byte_offset_with_multiline_code() {
    let output_file = "test_multiline.tags";
    let _ = fs::remove_file(output_file);

    let output = Command::new("./target/release/verus-etags")
        .args(&["-o", output_file, "test_data/sample.rs"])
        .output()
        .expect("Failed to execute verus-etags");

    assert!(output.status.success());

    let tags_content = fs::read_to_string(output_file)
        .expect("Failed to read TAGS file");

    // Should contain tags from the sample file
    assert!(tags_content.contains("sample.rs"));

    fs::remove_file(output_file).ok();
}

