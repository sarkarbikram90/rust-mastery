// String Slices
// A string slice is a reference to
// a contiguous sequence of the elements of a String,
// and it looks like this:

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn main() {
    let my_string = String::from("the quick brown fox jumps over the lazy dog");

    // first_word works with slices.
    let word = first_word(&my_string);

    println!("The first word is: {word}");

    let my_string_literal = "hello world";

    // first_word also works with string literals.
    let word = first_word(my_string_literal);

    println!("The first word is: {word}");
}