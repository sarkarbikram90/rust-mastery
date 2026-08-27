/*
fn main() {

    let a = 10;
    let b = 4;
    // arithmetic operations
    println!("Addition: {}", a + b);      // 14
    println!("Subtraction: {}", a - b);    // 6
    println!("Multiplication: {}", a * b);  // 40
    println!("Division: {}", a / b);      // 2 (integer division)
    println!("Modulo: {}", a % b);        // 2

    //  Floating Point Arithmetic
    //  Note: Floating point division truncates remainder, 
    //  But standard % operator is well-defined for floats in Rust

    let x = 10.0;
    let y = 4.0;

    println!("Float Division: {}", x / y);  // 2.5
    println!("Float Modulo: {}", x % y);    // 2.0
}
*/

// how to write/declare a function

// cook or
fn main() {
    println!("Rust is fun!");

     // go to kitchen
     another_function();
     third_function(); // another function

}

// make recipe
fn another_function() {
    println!("Rust is better than C++"); // what to make
}

fn third_function() {
    println!("Addition: {}", 1 + 2);
}


