// Sample Verus code to test etags generation
use vstd::prelude::*;

verus! {

// A spec function
spec fn factorial(n: nat) -> nat
    decreases n
{
    if n == 0 { 1 } else { n * factorial((n - 1) as nat) }
}

// A proof function
proof fn lemma_factorial_positive(n: nat)
    ensures factorial(n) > 0
    decreases n
{
    if n > 0 {
        lemma_factorial_positive((n - 1) as nat);
    }
}

// An exec function with specifications
fn compute_factorial(n: u64) -> (result: u64)
    requires n <= 20
    ensures result == factorial(n as nat)
{
    let mut i: u64 = 0;
    let mut acc: u64 = 1;
    while i < n
        invariant
            i <= n,
            acc == factorial(i as nat),
    {
        i = i + 1;
        acc = acc * i;
    }
    acc
}

// A struct
struct Counter {
    value: u64,
    ghost max: nat,
}

// Impl block
impl Counter {
    spec fn inv(&self) -> bool {
        self.value <= self.max
    }

    fn new(max: u64) -> (result: Self)
        ensures result.inv() && result.max == max
    {
        Counter { value: 0, ghost max: max as nat }
    }

    fn increment(&mut self)
        requires old(self).inv() && old(self).value < old(self).max
        ensures self.inv() && self.value == old(self).value + 1
    {
        self.value = self.value + 1;
    }
}

// A trait
trait Summable {
    spec fn sum(&self) -> int;
}

// An enum
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// Type alias
type MyResult<T> = Result<T, String>;

// Const
const MAX_SIZE: usize = 100;

// Static with ghost mode
static ghost GLOBAL_SPEC: int = 42;

// Broadcast group
broadcast group my_lemmas {
    lemma_factorial_positive,
}

} // verus!

