fn main() {
    // let reference_to_nothing = dangle();

    // let s = String::from("hello world");
    // let word = first_word(&s);

    // s.clear(); // This line would cause a compile-time error if uncommented, because `word` is still borrowing `s`.

    // println!("The first word is: {word}");

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];
    
    assert_eq!(slice, &[2, 3]);
}

fn dangle() -> String {
    let s = String::from("hello");
    s
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}