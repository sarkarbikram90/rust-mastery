// Methods
// Methods are similar to functions: 
// We declare them with the fn keyword 
// and a name, they can have parameters 
// and a return value, and they contain 
// some code that’s run when the method 
// is called from somewhere else. Unlike 
// functions, methods are defined within 
// the context of a struct 
// or an enum or a trait object
// and their first parameter is 
// always self, which represents the 
// instance of the struct the method 
// is being called on.

// Method Syntax
// the area function that has a 
// Rectangle instance as a parameter 
// and instead make an area method 
// defined on the Rectangle struct

#[derive(Debug)]
pub struct Rectangle {
    pub length: u32,
    pub width: u32,
}

impl Rectangle {
    pub fn area(&self) -> u32 {
        self.length * self.width
    }
}

pub fn main() {
    let rect1 = Rectangle {
        length: 50,
        width: 30,
    };

    println!(
        "The area of the rectangle is {}",
        rect1.area()
    );
}

