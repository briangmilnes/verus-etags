// Option 2: Struct wrapper for ordering functions
// Use this when passing ordering relations as function parameters

verus! {
    pub struct TotalOrderFn<T> {
        pub leq: spec_fn(T, T) -> bool,
    }
    
    impl<T> TotalOrderFn<T> {
        pub closed spec fn well_formed(&self) -> bool {
            // leq must be callable on any pair of T
            &&& forall|x: T, y: T| call_requires(self.leq, (&x, &y))
            // leq defines a total ordering
            &&& total_ordering(self.leq)
        }
        
        pub fn new(leq: spec_fn(T, T) -> bool) -> (result: Self)
            requires 
                total_ordering(leq),
                forall|x: T, y: T| call_requires(leq, (&x, &y))
            ensures result.well_formed()
        {
            TotalOrderFn { leq }
        }
        
        pub closed spec fn apply(&self, x: T, y: T) -> bool
            requires self.well_formed()
        {
            (self.leq)(x, y)
        }
        
        // Convenience method for comparing
        pub closed spec fn compare(&self, x: T, y: T) -> bool
            requires self.well_formed()
        {
            self.apply(x, y)
        }
    }
    
    // Example usage
    pub fn sorted<T>(items: &Vec<T>, ord: TotalOrderFn<T>) -> bool
        requires ord.well_formed()
    {
        forall|i: int, j: int| 
            0 <= i < j < items.len() ==> 
            ord.apply(items[i], items[j])
    }
    
    // Example: sorting function that takes an ordering
    pub fn sort_by<T>(items: &mut Vec<T>, ord: TotalOrderFn<T>)
        requires 
            ord.well_formed(),
            old(items).len() > 0
        ensures
            sorted(items, ord)
    {
        // Sorting implementation
    }
}


