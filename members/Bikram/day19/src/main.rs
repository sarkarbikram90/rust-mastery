// An Example Program Using Structs
// here we are not using structs
/*
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
*/

// here we are using structs
// Refactoring with Structs
// We use structs to add meaning by labeling the data.
/*
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}   
*/

// Refactoring with Tuples
// another version using tuples
fn main() {
    let rect1 = (30, 50); //tuple

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(rectangle: (u32, u32)) -> u32 {
    rectangle.0 * rectangle.1 // o position and 1 position
}

// In one way, this program is better.
// Tuples let us add a bit of structure,
// and we’re now passing just one argument.
// But in another way, this version is less clear:
// Tuples don’t name their elements, so we have to
// index into the parts of the tuple, 
// making our calculation less obvious.

