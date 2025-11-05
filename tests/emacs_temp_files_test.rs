use std::fs;
use std::process::Command;

#[test]
fn test_emacs_temp_files_ignored() {
    let test_dir = "test_emacs_temp";
    let _ = fs::remove_dir_all(test_dir);
    fs::create_dir(test_dir).expect("Failed to create test directory");
    
    // Create regular file
    fs::write(format!("{}/normal.rs", test_dir), "fn normal() {}").expect("Failed to create normal file");
    
    // Create Emacs temp files that should be ignored
    fs::write(format!("{}/.#locked.rs", test_dir), "fn locked() {}").expect("Failed to create lock file");
    fs::write(format!("{}/backup.rs~", test_dir), "fn backup() {}").expect("Failed to create backup file");
    fs::write(format!("{}/#autosave.rs#", test_dir), "fn autosave() {}").expect("Failed to create autosave file");
    
    let output_file = "test_emacs_temp.tags";
    let _ = fs::remove_file(output_file);
    
    // Run verus-etags
    let output = Command::new("./target/release/verus-etags")
        .args(&["-o", output_file, test_dir])
        .output()
        .expect("Failed to execute verus-etags");
    
    assert!(output.status.success(), "Command should succeed");
    
    // Read the TAGS file
    let tags_content = fs::read_to_string(output_file)
        .expect("Failed to read TAGS file");
    
    // Should only include normal.rs
    assert!(tags_content.contains("normal.rs"), "Should include normal.rs");
    assert!(!tags_content.contains(".#locked.rs"), "Should NOT include .#locked.rs");
    assert!(!tags_content.contains("backup.rs~"), "Should NOT include backup.rs~");
    assert!(!tags_content.contains("#autosave.rs#"), "Should NOT include #autosave.rs#");
    
    // Should find the normal function
    assert!(tags_content.contains("normal"), "Should find normal function");
    
    // Should NOT find temp file functions
    assert!(!tags_content.contains("locked"), "Should NOT find locked function from temp file");
    assert!(!tags_content.contains("backup"), "Should NOT find backup function from temp file");
    assert!(!tags_content.contains("autosave"), "Should NOT find autosave function from temp file");
    
    // Clean up
    fs::remove_file(output_file).expect("Failed to clean up TAGS file");
    fs::remove_dir_all(test_dir).expect("Failed to clean up test directory");
}

