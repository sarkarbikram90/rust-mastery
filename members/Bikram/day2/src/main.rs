/*
fn main() {
    let x = 5; // assign value 5
    println!("The value of x is: {x}");
    x = 6; // changing value to 6
    println!("The value of x is: {x}");
}
*/

// The above code will throw an error 
// as var value can't be changed
// To change the value of a variable, we need to use the 'mut' keyword
// 'mut' is short for mutable

fn main() {
    let mut x = 5; // assign value 5
    println!("The value of x is: {x}");
    x = 6; // changing value to 6
    println!("The value of x is: {x}");
}





