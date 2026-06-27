// In Rust, integers are numbers without a fractional component.
// They can be signed(+ve and -ve) or unsigned(+ve only).
// Integers have different lengths from 8-bit to 128-bit, and the default integer type is i32 and for Floats is f64.
// We also have arch dependent types isize and usize, which are used for indexing collections and pointer offsets.

// Tips: If we don't explicitly assign a type to a variable, then the compiler will infer one for us.

// Remove something to make it work
/* fn main() {
    let x: i32 = 5;
    let mut _y = 5; // i32 is inferred as the type of y

    _y = x;

    let _z: i32 = 10; // Type of z ?

    println!("Success!");
} */

// Fill the blank
/* fn main() {
    let _v: u16 = 38_u8 as u16;

    println!("Success!");
} */

// Tips: If we don't explicitly assign a type to a variable, then the compiler will infer one for us.
// RATE: ***
// Modify `assert_eq!` to make it work
/* fn main() {
    let x: u32 = 5;
    assert_eq!("u32".to_string(), type_of(&x));

    println!("Success!");
    println!("Type of x is {}", type_of(&x));
}
// Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
} */

// Fill the blanks to make it work
/* fn main() {
    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);

    println!("Success!");
}   */

// Fix errors and panics to make it work
/* fn main() {
    let v1: u16 = 251_u16 + 8;
    let v2: u16 = u16::checked_add(251, 8).unwrap();
    println!("{},{}", v1, v2);
} */

// Modify `assert!` to make it work
fn main() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111; // 1_024 + 255 + 63 + 255
    assert!(v == 1597);

    println!("Success!");
}
