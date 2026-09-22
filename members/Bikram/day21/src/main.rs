// Storing Lists of Values with Vectors
// Vec<T>, also known as a vector. 
// Vectors allow you to store more than one 
// value in a single data structure 
// that puts all the values next to each other 
// in memory. Vectors can only store 
// values of the same type. They are useful 
// when you have a list of items, 
// such as the lines of text in a file 
// or the prices of items in a shopping cart.
/*
fn main() {
    let v: Vec<i32> = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
}

// Using the macro vec! for Convenience
// A more common and concise way to create a vector 
// is to use the vec! macro.

fn main() {
    let v = vec![1, 2, 3, 4, 5];
}


// Reading Elements of a Vector
// We can access elements using square brackets and indexing,
// just like with strings.


fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");
}

*/

// Reading Elements of Vectors
/*
fn main() {

    let v = vec![1, 2, 3, 4, 5];        

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

}
*/
    // We use the index value of 2 to 
    // get the third element 
    // because vectors are 
    // indexed by number, starting at zero. 
    // Using & and [] gives us 
    // a reference to the element at the 
    // index value. When we use 
    // the get method with the index passed as an argument, 
    // we get an Option<&T> that we can use with match.

// Iterating Over the Values in a Vector
// To access each element in a 
// vector in turn, we would iterate 
// through all of the elements rather 
// than use indices to access one at a time.

fn main() {
    let v = vec![100, 32, 57];

    // Iterate over immutable references.
    for i in &v {
        println!("{i}");
    }

    // Iterate over mutable references
    // and modify each element.
    let mut v = vec![100, 32, 57];

    for i in &mut v {
        *i += 50;
    }

    println!("{v:?}");
}