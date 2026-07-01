/*
fn main() {
    // Use as many approaches as you can to make it work
    let x: String = String::from("Hello world");
    let y: String = x.clone(); // Deep copying
    println!("{}, {}", x, y);
}
*/
/*
// Don't modify code in main!
fn main() {
    let s1: String = String::from("Hello world");
    let s2: String = take_ownership(s1);

    println!("{}", s2);
}

// Only modify the code below!
fn take_ownership(s: String) -> String {
    println!("{}", s);
    s
}
*/
/*
fn main() {
    let s: String = give_ownership();
    println!("{}", s);
}

// Only modify the code below!
fn give_ownership() -> String {
    let s: String = String::from("Hello world");
    // Convert String to Vec
    let _s = s.as_bytes();
    s
}
*/

/*
// Fix the error without removing any code
fn main() {
    let s: String = String::from("Hello World");

    print_str(s.clone());

    println!("{}", s);
}

fn print_str(s: String) {
    println!("{}", s)
}
*/

// Don't use clone ,use copy instead
fn main() {
    let x: (i32, i32, (), &str) = (1, 2, (), "hello");
    let y: (i32, i32, (), &str) = x;
    println!("{:?}, {:?}", x, y);
}
