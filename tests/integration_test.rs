use std::fs;
use std::process::Command;

#[test]
fn test_verus_integers_example() {
    let verus_examples = std::env::var("HOME").unwrap() + "/projects/VerusCodebases/verus/examples";
    let example_file = format!("{}/integers.rs", verus_examples);
    
    if !std::path::Path::new(&example_file).exists() {
        eprintln!("Skipping test: {} not found", example_file);
        return;
    }

    let output_tags = "test_verus_integers.tags";
    let _ = fs::remove_file(output_tags);

    let output = Command::new("./target/release/verus-etags")
        .args(&["-o", output_tags, &example_file])
        .output()
        .expect("Failed to execute verus-etags");

    assert!(
        output.status.success(),
        "verus-etags failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    
    // Read the generated TAGS file
    let tags_content = fs::read_to_string(output_tags)
        .expect("Failed to read generated TAGS file");
    
    // Verify key tags are present (now that we process verus! macros)
    // Check for spec functions
    assert!(
        tags_content.contains("add1_int"),
        "Missing tag for spec function add1_int"
    );
    assert!(
        tags_content.contains("add1_nat"),
        "Missing tag for spec function add1_nat"
    );
    assert!(
        tags_content.contains("add1_nat_opaque"),
        "Missing tag for spec function add1_nat_opaque"
    );
    
    // Check for proof functions
    assert!(
        tags_content.contains("test0"),
        "Missing tag for proof function test0"
    );
    assert!(
        tags_content.contains("test1"),
        "Missing tag for proof function test1"
    );
    
    // Check for Verus-specific functions
    assert!(
        tags_content.contains("add1_int"),
        "Missing tag for spec function add1_int"
    );
    assert!(
        tags_content.contains("add1_u64"),
        "Missing tag for exec function add1_u64"
    );
    
    // Clean up
    fs::remove_file(output_tags).ok();
}

#[test]
fn test_simple_rust_file() {
    let output_tags = "test_simple.tags";
    let _ = fs::remove_file(output_tags);

    let output = Command::new("./target/release/verus-etags")
        .args(&["-o", output_tags, "test_data/simple.rs"])
        .output()
        .expect("Failed to execute verus-etags");

    assert!(
        output.status.success(),
        "verus-etags failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let tags_content = fs::read_to_string(output_tags)
        .expect("Failed to read TAGS file");

    // Check for basic tags
    assert!(tags_content.contains("regular_function"));
    assert!(tags_content.contains("spec_function"));
    assert!(tags_content.contains("proof_function"));
    assert!(tags_content.contains("MyStruct"));
    assert!(tags_content.contains("MyEnum"));

    // Clean up
    fs::remove_file(output_tags).ok();
}

#[test]
fn test_verus_recursion_example() {
    let verus_examples = std::env::var("HOME").unwrap() + "/projects/VerusCodebases/verus/examples";
    let example_file = format!("{}/recursion.rs", verus_examples);
    
    if !std::path::Path::new(&example_file).exists() {
        eprintln!("Skipping test: {} not found", example_file);
        return;
    }

    let output_tags = "test_verus_recursion.tags";
    let _ = fs::remove_file(output_tags);

    let output = Command::new("./target/release/verus-etags")
        .args(&["-o", output_tags, &example_file])
        .output()
        .expect("Failed to execute verus-etags");

    assert!(
        output.status.success(),
        "verus-etags failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    
    let tags_content = fs::read_to_string(output_tags)
        .expect("Failed to read generated TAGS file");
    
    // Check for Verus-specific spec function
    assert!(
        tags_content.contains("arith_sum_int"),
        "Missing tag for spec function arith_sum_int"
    );
    assert!(
        tags_content.contains("arith_sum_u64"),
        "Missing tag for exec function arith_sum_u64"
    );
    assert!(
        tags_content.contains("arith_sum_int_nonneg"),
        "Missing tag for proof function arith_sum_int_nonneg"
    );
    assert!(
        tags_content.contains("arith_sum_test1"),
        "Missing tag for proof function arith_sum_test1"
    );
    
    // Clean up
    fs::remove_file(output_tags).ok();
}

#[test]
fn test_append_mode() {
    let output_tags = "test_append.tags";
    let _ = fs::remove_file(output_tags);

    // Generate initial tags
    Command::new("./target/release/verus-etags")
        .args(&["-o", output_tags, "test_data/simple.rs"])
        .output()
        .expect("Failed to execute verus-etags");

    // Append more tags
    let output = Command::new("./target/release/verus-etags")
        .args(&["-a", "-o", output_tags, "test_data/another.rs"])
        .output()
        .expect("Failed to execute verus-etags");

    assert!(output.status.success());

    // Clean up
    fs::remove_file(output_tags).ok();
}

#[test]
fn test_sorting() {
    let output_tags = "test_sorting.tags";
    let _ = fs::remove_file(output_tags);

    let output = Command::new("./target/release/verus-etags")
        .args(&["-s", "1", "-o", output_tags, "test_data/simple.rs"])
        .output()
        .expect("Failed to execute verus-etags");

    assert!(output.status.success());

    // Clean up
    fs::remove_file(output_tags).ok();
}

