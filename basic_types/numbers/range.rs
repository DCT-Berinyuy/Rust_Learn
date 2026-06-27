// Two goals: 1. Modify assert! to make it work 2. Make println! output list of numbers between 97 and 122

/* fn main() {
    let mut sum: i32 = 0;
    for i in -3..2 {
        // -3, -2, -1, 0, 1. The nth term is always excluded, so 2 is not included in the range.
        sum += i
    }

    assert!(sum == -5);

    for c in 'a'..='z' {
        // Here we use the inclusive range operator `..=` to include 'z' in the range.
        println!("{}", c as u8);
    }
} */

// Fill the blanks
use std::ops::{Range, RangeInclusive};
fn main() {
    assert_eq!((1..5), Range { start: 1, end: 5 }); // The `Range` struct represents a range of values from `start` (inclusive) to `end` (exclusive).
    assert_eq!((1..=5), RangeInclusive::new(1, 5)); // The `RangeInclusive` struct represents a range of values from `start` (inclusive) to `end` (inclusive).

    println!("Success!");
}
