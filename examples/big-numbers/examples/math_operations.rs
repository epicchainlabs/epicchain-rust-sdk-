use std::ops::{Div, Mul};

use primitive_types::U256;

/// `U256` implements traits in `std::ops`, that means it supports arithmetic operations
/// using standard Rust operators `+`, `-`. `*`, `/`, `%`, along with additional utilities to
/// perform common mathematical tasks.
fn main() {
    let a = U256::from(10);
    let b = U256::from(2);

    // addition
    let sum = a + b;
    assert_eq!(sum, U256::from(12));

    // subtraction
    let difference = a - b;
    assert_eq!(difference, U256::from(8));

    // multiplication
    let product = a * b;
    assert_eq!(product, U256::from(20));

    // division
    let quotient = a / b;
    assert_eq!(quotient, U256::from(5));

    // modulo
    let remainder = a % b;
    assert_eq!(remainder, U256::zero()); // equivalent to `U256::from(0)`

    // exponentiation
    let power = a.pow(b);
    assert_eq!(power, U256::from(100));
    // powers of 10 can also be expressed like this:
    let power_of_10 = U256::exp10(2);
    assert_eq!(power_of_10, U256::from(100));


    let gas1 = U256::from(1_000_000_000_u32); // 10 gas
    let gas2 = U256::from(2_000_000_000_u32); // 20 gas
    let base = U256::from(10).pow(8.into());
    let mul = gas1.mul(gas2).div(base.pow(2.into()));
    assert_eq!(mul, U256::from(200)); // 200
}
