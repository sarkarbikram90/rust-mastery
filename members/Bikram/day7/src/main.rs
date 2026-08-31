// References and Borrowing
// this wont compile
// Just as variables are immutable by default, 
// so are references. 
// We’re not allowed to modify something we have a reference to.
/*
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}
*/
// Mutable References
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

/*
let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{r1}, {r2}");
    */