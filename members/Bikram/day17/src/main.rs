// The Slice Type
// Slices let you reference a contiguous 
// sequence of elements in a collection. 
// A slice is a kind of reference, 
// so it does not have ownership.

// Problem statement:
// Write a function that takes a string of 
// words separated by spaces and returns 
// the first word it finds in that string. 
// If the function doesn’t find a space in 
// the string, the whole string must be 
// one word, so the entire string should be returned.
fn main() {
    let s = String::from("hello world");

    let word = first_word(&s);

    println!("The first word is: {}!", word);
}

fn first_word(s: &str) -> &str {
    // Because we need to go through the 
    // String element by element and 
    // check whether a value is a space, 
    // we’ll convert our String to an 
    // array of bytes using the as_bytes method.
    let bytes = s.as_bytes();
    // Next, we create an iterator over the array 
    // of bytes using the iter method:
    for (i, &item) in bytes.iter().enumerate() {
         // Inside the for loop, we search for the byte that represents the 
         // space by using the byte literal syntax. If we find a space, 
         // we return the position. Otherwise, we return the length 
         // of the string.
        
        if item == b' ' {  // is a byte literal.
            return &s[..i]; // This is a string slice.
        }
    }
    // This means Return a slice containing the entire string
    // returns the entire string.
    &s[..] 
} 

/*
// Rust-y piece to internalize is:
.iter()        // gives references
.enumerate()   // adds indexes
&i             // destructures a reference in a pattern
b' '           // byte literal
&s[..i]        // borrowed string slice

// Refer to https://doc.rust-lang.org/book/ch04-03-slices.html
*/
