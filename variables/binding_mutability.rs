/*
 * ### Binding and mutability ###
 * Assigned using 'let' keyword
 * Print to standard output by 'print!() or println!()
 * 'ln' is for NewLine
 */

// A variable can be used only if it has been initialized.

// Fix the error below with least amount of modification to the code

/* fn main() {
    let x: i32 = 5; // Uninitialized but used, ERROR !
    let _y: i32; // Uninitialized but also unused, only a Warning !

    assert_eq!(x, 5);
    println!("Success!");
} */

// Use mut to mark a variable as mutable.
// In rust, a variable is immutable in default.

// Fill the blanks in the code to make it compile
fn main() {
    let mut x: i32 = 1;
    x += 2;

    assert_eq!(x, 3);
    println!("Success!");
}
