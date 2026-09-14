// The match Control Flow Construct
// Rust has an extremely powerful
// control flow construct called match
// that allows you to compare a value
// against a series of patterns and then
// execute code based on which pattern matches.

fn main() {
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("something else"),
    }
}
// The match Control Flow Construct
// match is an expression,
// so the branch that matches the value
// is the value of the whole match expression.
// Example:
fn plus_one(x: i32) -> i32 {
    match x {
        1 => 1,
        2 => 2,
        _ => 3,
    }
}


// In this example,
// match returns 1, 2, or 3
// depending on the value of x.  

// example using match! 
// We can write a function
// that takes an unknown US coin and
// determines which coin it is and returns
// its value in cents

pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

// the match in the value_in_cents function. 
// First, we list the match keyword followed 
// by an expression, which in this 
// case is the value coin. 
// This seems very similar to a conditional expression 
// used with if, but there’s a big difference:
// With if, the condition needs to 
// evaluate to a Boolean value, 
// but here it can be any type. 
// The type of coin in this example 
// is the Coin enum 
// that we defined on the first line.

fn value_in_centsmain(coin: Coin) -> u8 {
    match coin {
        // Next are the match arms. 
        // An arm has two parts: 
        // a pattern and some code. 
        // The first arm here has a 
        // pattern that is the value Coin::Penny 
        // and then the => operator 
        // that separates the pattern and the 
        // code to run. 
        // The code in this case is just the 
        // value 1. 
        // Each arm is separated from the 
        // next with a comma.
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let coin = Coin::Quarter;
    println!("{}", value_in_centsmain(coin));
}