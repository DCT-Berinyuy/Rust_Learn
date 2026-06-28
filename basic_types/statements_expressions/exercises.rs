// Exercises

// Make it work with two ways
/* fn main() {
    let v: i32 = {
        let mut x: i32 = 1;
        x += 2;
        x
    };

    assert_eq!(v, 3);

    println!("Success!");
} */

/* fn main() {
    let v: i32 = {
        let x = 3;
        x
    };

    assert!(v == 3);

    println!("Success!");
} */

fn main() {
    let s: i32 = sum(1, 2); // s = 3
    assert_eq!(s, 3);

    println!("Success!");
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}
