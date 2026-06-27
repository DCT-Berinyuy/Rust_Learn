// You can declare a new variable with the same name as a previous variable,
// here we can say the first one is shadowed by the second one

// Only modify `assert_eq!` to make the `println!` work(print `42` in terminal)
/* fn main() {
    let x: i32 = 5;
    {
        let x = 12; // This `x` is in the inner scope and shadows the outer `x`.
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x = 42; // This `x` shadows the previous `x`.
    println!("{}", x); // Prints "42".
} */

// Remove a line in the code to make it compile
fn main() {
    let mut _x: i32 = 1;
    _x = 7;
    // Shadowing and re-binding
    _x += 3;

    let _y: i32 = 4;
    // Shadowing
    let _y: &str = "I can also be bound to text!";

    println!("Success!");
}
