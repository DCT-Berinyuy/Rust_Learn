// Statement: Instructs that perform some action and do not return a value and end with a semicolon
// Functions, loops, and variable declarations are examples of statements in Rust. They are used to control the flow of the program and perform actions, but they do not produce a value that can be used in further expressions.
// Expression: A piece of code that evaluates to a value
// Examples of expressions in Rust include literals, variables, function calls, and operators.

// Examples

fn main() {
    let x: u32 = 5u32;

    let y: u32 = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y`
        x_cube + x_squared + x
    };

    let z: u32 = {
        // The semicolon suppresses this expression and `()` is assigned to `z`
        2 * x
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}
