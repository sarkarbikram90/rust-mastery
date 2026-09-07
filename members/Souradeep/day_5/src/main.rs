fn main() {
    // let s1 = String::from("hello");
    // let s2 = s1; // s1 is moved to s2, s1 is no longer valid

    // println!("{s1}, world!"); // this will cause a compile-time error because s1 is no longer valid
    
    // scope and assignment
    let mut s = String::from("hello");
    s = String::from("ahoy");

    println!("{s}, world!");

    // clone function 
    let s1 = String::from("hello");
    let s2 = s1.clone();

    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {len}.");

    println!("s1 = {s1}, s2 = {s2}");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}