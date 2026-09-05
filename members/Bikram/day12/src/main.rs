// Defining and Instantiating Structs
// To define a struct, we enter the keyword 
// struct and name the entire struct. 
// A struct’s name should describe the significance 
// of the pieces of data being grouped together. 
// Then, inside curly brackets, we define the names 
// and types of the pieces of data, which we call fields.

#[derive(Debug)] //To print the struct
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    //Creating an instance of the struct
    let user1 = User {
        email: String::from("user1@example.com"),
        username: String::from("user1"),
        active: true,
        sign_in_count: 7,
    };

    let user2 = User {
        email: String::from("user2@example.com"),
        username: String::from("user2"),
        active: false,
        sign_in_count: 5,
    };

    //Accessing the values of the struct
    println!("User 1 {:?}\n", user1);      // Compact debug
    println!("User 1 {:#?}\n", user1);     // Pretty-printed debug
    println!("User 1 {:?}\n", user1.email);  // Accessing specific fields
    println!("User 1 {:?}\n", user1.username);  // Accessing specific fields
    println!("User 1 {:?}\n", user1.active);  // Accessing specific fields
    println!("User 1 {:?}\n", user1.sign_in_count);  // Accessing specific fields    

    println!("\nUser 2 {:?}\n", user2);      // Compact debug
    println!("User 2 {:#?}\n", user2);     // Pretty-printed debug
    println!("User 2 {:?}\n", user2.email);  // Accessing specific fields
    println!("User 2 {:?}\n", user2.username);  // Accessing specific fields
    println!("User 2 {:?}\n", user2.active);  // Accessing specific fields
    println!("User 2 {:?}\n", user2.sign_in_count);  // Accessing specific fields    
    
}

    
