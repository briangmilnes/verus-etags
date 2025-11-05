// Simple test file

fn regular_function() {
    println!("Hello");
}

spec fn spec_function(x: int) -> int {
    x + 1
}

proof fn proof_function(x: int) 
    ensures x >= 0
{
}

struct MyStruct {
    field: i32,
}

enum MyEnum {
    Variant1,
    Variant2(i32),
}

trait MyTrait {
    fn trait_method(&self);
}

impl MyStruct {
    fn new() -> Self {
        MyStruct { field: 0 }
    }
}

const MY_CONST: i32 = 42;

static MY_STATIC: i32 = 100;

type MyType = Vec<i32>;

mod my_module {
    fn inner_function() {}
}

