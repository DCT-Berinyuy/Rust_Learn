// Diverging functions
// Diverging functions never return to the caller, so they may be used in places where a value of any type is expected.
// For example, the `panic!` macro is a diverging function, and it can be used in a function that returns any type.

/*
fn main() {
    println!("Success!");
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        // match is similar to switch in other languages, it allows you to match on a value and execute code based on the value
        1 => {
            // TODO
        }
        _ => {
            // TODO
        }
    };

    // Rather than returning a None, we use a diverging function instead
    never_return_fn()
}

// IMPLEMENT this function in THREE ways
fn never_return_fn() -> ! {
    unimplemented!() // This is a macro that will panic with a message that the function is not implemented
                     // Other ways to implement this function:
                     // panic!("This function never returns!");
                     // todo!(): Implement a loop that never returns
}
*/

fn main() {
    // FILL in the blank
    let b: bool = false;

    let _v = match b {
        true => 1,
        // Diverging functions can also be used in match expression to replace a value of any value
        false => {
            println!("Success!");
            panic!("we have no value for `false`, but we can panic");
        }
    };

    println!("Exercise Failed if printing out this line!");
}
