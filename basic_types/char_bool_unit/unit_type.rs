// A unit type in Rust is a type that has exactly one value, which is also called the unit value. The unit type is denoted by `()`, and it is used to indicate that a function does not return any meaningful value. In other words, when a function returns the unit type, it means that the function has no return value.
// Make it work, don't modify `implicitly_ret_unit` !
/* fn main() {
    let _v: () = (); // A unit type value is represented by an empty tuple `()`. It is used to indicate that a function does not return any meaningful value.

    let v: (i32, i32) = (2, 3);
    assert_eq!(_v, implicitly_ret_unit());

    println!("Success!");
} */

/* fn implicitly_ret_unit() {
    println!("I will return a ()");
} */

// Don't use this one
/* fn explicitly_ret_unit() -> () {
    println!("I will return a ()");
}
*/

// 🌟🌟 What's the size of the unit type?

// Modify `4` in assert to make it work
use std::mem::size_of_val;
fn main() {
    let unit: () = ();
    assert!(size_of_val(&unit) == 0);
    println!("Size of unit type: {} bytes", size_of_val(&unit));

    println!("Success!");
}
