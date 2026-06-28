// Definition: A `char` in Rust is a Unicode scalar value, which is a 32-bit value.
// Make it work
// use std::mem::size_of_val;
/* fn main() {
    let c1: char = 'a';
    assert_eq!(size_of_val(&c1), 4);
    println!("Size of c1: {}", size_of_val(&c1));
    let c2: char = '中';
    assert_eq!(size_of_val(&c2), 4);

    println!("Success!");
} */

// Make it work
fn main() {
    let c1: char = '中';// Single quotes are used for char literals in Rust
    print_char(c1);
}

fn print_char(c: char) {
    println!("{}", c);
}
