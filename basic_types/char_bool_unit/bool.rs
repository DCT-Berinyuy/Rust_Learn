// Make println! work
/* fn main() {
    let _f: bool = false;

    let t: bool = true;
    if t {
        println!("Success!");
    }
} */

// Make it work
fn main() {
    let f: bool = false;
    let t: bool = true && false;// The && operator is used to perform a logical AND operation between two boolean values
    assert_eq!(t, f);

    println!("Success!");
}
