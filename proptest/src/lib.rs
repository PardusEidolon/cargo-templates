#![allow(unused)]

use proptest::prelude::*;

pub fn add_a_b(a: u32, b: u32) -> u32 {
    a + b
}

// See documentation:  for different strategies
// use `proptest` for testing with builtin strategies
// use `propcompose` for building your own strategies to test with
proptest! {
    // types can be generated on the fly based on primitive types.
    #[test]
    fn test_add(a in 0..100u32, b in 0..100u32) {
        let sum = add_a_b(a, b);

        assert!(a <= sum);
        assert!(b <= sum);
    }
    // Arbitrary chars of variable lengths including unicode is generated.
    #[test]
    fn gen_acii_uni(s in "\\PC*" ) {
        dbg!(s);
    }
    // Arbitrary chars of variable lengths is generated.
    #[test]
    fn gen_ascii(s in ".*" ) {
        dbg!(s);
    }
}
