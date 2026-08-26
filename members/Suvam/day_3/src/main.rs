// always print mut borrow first and then mut owner (Why it is like this)
fn main() {
    // === 1. OWNERSHIP ===
    // string_owner is created and allocated on the heap. 
    //  let second_borrower = &string_owner; It is the sole owner.
    let string_owner = String::from("Rust");

    // === 2. BORROWING ===
    // We pass string_owner immutably by reference using '&'.
    // The value is borrowed without taking ownership.
    let borrower = &string_owner; 
    
    // string_owner is still valid!
    println!("The value of '{}' is {}", string_owner, borrower); 
    
    // mutable borrowing of string_owner_1
    let mut string_owner_1 = String::from("Rust1");
    
    //mutable borrow of string_owner_1
    let mut_borrow = &mut string_owner_1;
    // change value of string_owner_1 using mutable borrow
    mut_borrow.push_str(" Rust2");
    
    // always print mut borrow first and then mut owner
    println!("Borrower: {}", mut_borrow);
    println!("Owner: {}", string_owner_1);
    
}

/*
    // Ownership is now MOVED from string_owner to mut_string.
    // string_owner is invalidated to prevent "double-free" memory bugs.
    let mut mut_string = string_owner; 
    // println!("{}", string_owner); // ERROR! Compile-time check blocks using moved values.

    // We borrow mut_string mutably using '&mut' to modify its data.
    modify_string(&mut mut_string); 

    // === 3. LIFETIMES ===
    let result;
    {
        let short_lived = String::from("Safety");
        
        // We compare the two strings. The compiler checks their lifetimes.
        result = longest(&mut_string, &short_lived);
        
        println!("The longest string inside the scope is: {}", result); // OK! Both strings are alive.
    } 
    // println!("{}", result); // ERROR! The compiler blocks this because 'short_lived' was dropped,
                               // which would leave 'result' as a dangling reference.
}

// Accepts an immutable reference (borrow) of a String
fn calculate_length(s: &String) -> usize {
    s.len()
}

// Accepts a mutable reference (borrow) to modify the string
fn modify_string(s: &mut String) {
    s.push_str(" language");
}

// Function with a generic lifetime parameter 'a.
// It specifies that the returned reference lives only as long as the SHORTEST of x and y.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

*/

