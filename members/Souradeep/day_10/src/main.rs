// struct User {
//     active: bool,
//     username: &str,
//     email: &str,
//     sign_in_count: u64,
// }
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // let user1 = User {
    //     active: true,
    //     username: String::from("someusername123"),
    //     email: String::from("someone@example.com"),
    //     sign_in_count: 1,
    // };
    let scale = 2;

    let rect1 = Rectangle {
        width: dbg!(30*scale),
        height: 50,
    };

    dbg!(&rect1);

    // println!("rect1 is {rect1:#?}");

    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(&rect1)
    // );
}

// fn build_user(email: String, username: String) -> User {
//     User {
//         active: true,
//         username,
//         email,
//         sign_in_count: 1,
//     }
// }

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
