// We can use a pattern with let to destructure a tuple to separate variables.
// Tips: you can use Shadowing or Mutability

// Fix the error below with least amount of modification
/* fn main() {
    let (mut x, y) = (1, 2); // Destructuring a tuple into two variables `x` and `y`
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
} */

fn main() {
    let (x, y);
    (x, ..) = (3, 4);
    [.., y] = [1, 2];
    // Fill the blank to make the code work
    assert_eq!([x, y], [3, 2]);

    println!("Success!");
}
