// Functions: Reusable blocks of code that perform a specific task
// Can take arguments and return values
// Diverging functions: Functions that never return a value, they either loop forever or panic

/* fn main() {
    // Don't modify the following two lines!
    let (x, y) = (1, 2);
    let s: i32 = sum(x, y); // The caller

    assert_eq!(s, 3);

    println!("Success!");
}

fn sum(x: i32, y: i32) -> i32 {
    // The callee
    x + y
} */

/* fn main() {
    print(); // The caller
}

// Replace i32 with another type
fn print() {
    println!("Success!");
} */

// Solve it in two ways
// DON'T let `println!` work
fn main() {
    never_return();

    println!("Failed!");
}

fn never_return() -> ! {
    // diverging function, never returns
    // Implement this function, don't modify the fn signatures
    panic!("This function never returns!");
}
