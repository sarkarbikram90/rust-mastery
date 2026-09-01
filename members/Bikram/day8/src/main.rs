// Data Types
// integar signed and unsigned
// i is signed meaning, it can hold both positive and negative values
// u is unsigned meaning, it can hold only positive values
// in Rust default integer data type is i32
// u8 can hold value from 0 to 255

fn main() {
    // here u8 is greater than 255 so it will throw an error
    // let x: (i32, f64, u8) = (500, 6.4, 351); 
    // it will compile now as u8 value is less than 255
    let x: (i32, f64, u8) = (500, 6.4, 15); 

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    println!("The value of five_hundred is: {five_hundred}");
    println!("The value of six_point_four is: {six_point_four}");
    println!("The value of one is: {one}");
}