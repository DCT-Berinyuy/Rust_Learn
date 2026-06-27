// Floating-point numbers are numbers with a fractional component.
// They can be signed(+ve and -ve) or unsigned(+ve only).
// Rust provides two floating-point types: f32 and f64.
// The default floating-point type is f64.

/* fn main() {
    let x: f64 = 3.14;
    let y = 2.0; // f64 is inferred as the type of y

    println!("x: {}, y: {}", x, y);
} */

// Fill the blank to make it work
/*
 fn main() {
    let x: f64 = 1_000.000_1; // ?
    let _y: f32 = 0.12; // f32
    let _z: f64 = 0.01_f64; // f64

    assert_eq!(type_of(&x), "f64".to_string());
    println!("Success!");
    println!("Type of x is {}", type_of(&x));
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
*/

// Make it work in two distinct ways

fn main() {
    assert!(0.1 as f32 + 0.2_f32 == 0.3_f32); // This will fail due to floating-point precision issues

    println!("Success!");
}
