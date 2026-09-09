// Storing Lists of Values with Vectors
// Creating a New Vector
// To create a new, empty vector, 
// we call the Vec::new function
/*
fn main() {
    let v: Vec<i32> = Vec::new();
}
*/
// Creating a Vector with Macro
// We can use the vec! macro to create a vector and 
// populate it with initial values. The macro 
// makes it easy to create a vector with the 
// values you want to start with.

// Updating a Vector
// To create a vector and then add elements to it, 
// we can use the push method
fn main() {
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);

    println!("The vector is: {:?}!", v);
}

// Refer to https://doc.rust-lang.org/book/ch08-01-vectors.html



