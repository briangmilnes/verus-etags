use std::fs;
use std::process::Command;

#[test]
fn test_verus_syntax_types() {
    let test_file = "test_verus_syntax.rs";
    
    // Create a test file with various Verus syntax constructs
    let test_content = r#"
verus! {

// Spec function
spec fn my_spec(x: int) -> int {
    x + 1
}

// Proof function
proof fn my_proof(x: int) {
    assert(x == x);
}

// Exec function
fn my_exec(x: u32) -> u32 {
    x + 1
}

// Spec const
spec const SPEC_CONST: int = 42;

// Exec const
exec const EXEC_CONST: u64
    ensures EXEC_CONST == 100,
{
    100
}

// Regular const
const REGULAR_CONST: u8 = 5;

// Struct
pub struct MyStruct {
    field: u32,
}

// Enum
pub enum MyEnum {
    Variant1,
    Variant2(u32),
}

// Trait
pub trait MyTrait {
    fn trait_fn(&self) -> u32;
}

// Impl
impl MyStruct {
    fn new() -> Self {
        MyStruct { field: 0 }
    }
}

// Module
mod my_module {
    pub fn inner_fn() {}
}

// Type alias
type MyType = u32;

// Open spec
pub open spec fn open_spec_fn(x: int) -> int {
    x * 2
}

} // verus!
"#;
    
    fs::write(test_file, test_content).expect("Failed to create test file");
    
    let output_tags = "test_verus_syntax.tags";
    let _ = fs::remove_file(output_tags);
    
    // Run verus-etags
    let output = Command::new("./target/release/verus-etags")
        .args(&["-o", output_tags, test_file])
        .output()
        .expect("Failed to execute verus-etags");
    
    assert!(
        output.status.success(),
        "verus-etags failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    
    // Read generated TAGS
    let tags_content = fs::read_to_string(output_tags)
        .expect("Failed to read TAGS file");
    
    // Verify each syntax type is tagged
    assert!(tags_content.contains("my_spec"), "Missing spec fn tag");
    assert!(tags_content.contains("my_proof"), "Missing proof fn tag");
    assert!(tags_content.contains("my_exec"), "Missing exec fn tag");
    assert!(tags_content.contains("SPEC_CONST"), "Missing spec const tag");
    assert!(tags_content.contains("EXEC_CONST"), "Missing exec const tag");
    assert!(tags_content.contains("REGULAR_CONST"), "Missing regular const tag");
    assert!(tags_content.contains("MyStruct"), "Missing struct tag");
    assert!(tags_content.contains("MyEnum"), "Missing enum tag");
    assert!(tags_content.contains("MyTrait"), "Missing trait tag");
    assert!(tags_content.contains("new"), "Missing impl fn tag");
    assert!(tags_content.contains("my_module"), "Missing module tag");
    assert!(tags_content.contains("MyType"), "Missing type alias tag");
    assert!(tags_content.contains("open_spec_fn"), "Missing open spec fn tag");
    
    // Verify patterns include the keywords
    assert!(tags_content.contains("spec fn my_spec"), "Pattern missing 'spec fn'");
    assert!(tags_content.contains("proof fn my_proof"), "Pattern missing 'proof fn'");
    assert!(tags_content.contains("spec const SPEC_CONST"), "Pattern missing 'spec const'");
    assert!(tags_content.contains("exec const EXEC_CONST"), "Pattern missing 'exec const'");
    assert!(tags_content.contains("pub struct MyStruct"), "Pattern missing 'pub struct'");
    assert!(tags_content.contains("pub enum MyEnum"), "Pattern missing 'pub enum'");
    assert!(tags_content.contains("pub trait MyTrait"), "Pattern missing 'pub trait'");
    assert!(tags_content.contains("mod my_module"), "Pattern missing 'mod'");
    assert!(tags_content.contains("pub open spec fn open_spec_fn"), "Pattern missing 'pub open spec fn'");
    
    // Clean up
    fs::remove_file(test_file).ok();
    fs::remove_file(output_tags).ok();
}

