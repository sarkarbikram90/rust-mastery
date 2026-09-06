
// Dangling References:
// In languages with pointers, 
// it’s easy to erroneously create 
// a dangling pointer, 
// a pointer that references 
// a location in memory that may have 
// been given to someone else, 
// by freeing some memory while preserving 
// a pointer to that memory.    

// The Borrow Checker:
// Rust prevents dangling references 
// by means of a borrow checker, 
// a set of rules that 
// the compiler checks at compile time.

fn main() {
    // let reference_to_nothing = dangle(); // This will not compile
    let reference_to_nothing = no_dangle();
    println!("{reference_to_nothing}");
}

/*
fn dangle() -> &String { // dangle returns a reference to a String
    let s = String::from("hello"); // s is a new String

    &s  // we return a reference to the String, s
        // Here, s goes out of scope and is dropped,
        // so its memory goes away.
        // Danger!
}

// Because s is created inside dangle, 
// when the code of dangle is finished, 
// s will be deallocated. 
// But we tried to return a reference to it.
// That means this reference would be pointing 
// to an invalid String. That’s no good!
// Rust won’t let us do this.

// The solution here is to return the String directly:

*/
fn no_dangle() -> String {
    let s = String::from("hello");

    s // We return s itself, not a reference to it.
}

// This works without any problems. 
// Ownership is moved out, 
// and nothing is deallocated.


// The Rules of References
// 1. At any given time, 
// you can have either one mutable reference or 
// any number of immutable references.
// 2. References must always be valid.



