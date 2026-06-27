// Fill the blanks and fix the errors
fn main() {
    // Integer addition
    assert!(1u32 + 2 == 3); // The compiler will infer the type of the expression based on the types of the operands. In this case, both operands are of type `u32`, so the result will also be of type `u32`.

    // Integer subtraction
    assert!(1i32 - 2 == -1); // The compiler will infer the type of the expression based on the types of the operands. In this case, both operands are of type `i32`, so the result will also be of type `i32`.
    assert!(1i8 - 2 == -1);

    assert!(3 * 50 == 150); // Default type is i32, so the result will also be of type i32.

    assert!(9.6 as f32 / 3.2 as f32 == 3.0_f32); // error ! make it work

    assert!(24 % 5 == 4);
    // Short-circuiting boolean logic
    assert!(true && false == false); // The `&&` operator is a short-circuiting operator, which means that if the left-hand side of the expression is `false`, the right-hand side will not be evaluated. In this case, since the left-hand side is `true`, the right-hand side will be evaluated, and the result will be `false`.
    assert!(true || false == true); // The `||` operator is a short-circuiting operator, which means that if the left-hand side of the expression is `true`, the right-hand side will not be evaluated. In this case, since the left-hand side is `true`, the right-hand side will not be evaluated, and the result will be `true`.
    assert!(!true == false); // The `!` operator is a unary operator that negates the value of the operand. In this case, since the operand is `true`, the result will be `false`.

    // Bitwise operations
    // Definition: Bitwise operations are operations that directly manipulate bits of binary numbers.
    // They are performed on the binary representations of integers and include AND, OR, XOR, NOT, left shift, and right shift operations.
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}
