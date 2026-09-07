// Smart pointers
// A pointer is a general concept for a variable that 
// contains an address in memory. This address refers 
// to, or “points at,” some other data. The most common 
// kind of pointer in Rust is a reference, which you 
// learned about in Chapter 4. References are indicated 
// by the & symbol and borrow the value they point to.
// They don’t have any special capabilities other than 
// referring to data, and they have no overhead.

// Using Box<T> to Point to Data on the Heap
// The most straightforward smart pointer is a box, 
// whose type is written Box<T>. Boxes allow you to 
// store data on the heap rather than the stack.
// What remains on the stack is the pointer to the heap data.

// Boxes don’t have performance overhead, other than
// storing their data on the heap instead of on the stack. 
// But they don’t have many extra capabilities either. 
// You’ll use them most often in these situations:

// When you have a type whose size can’t be known at compile time, 
// and you want to use a value of that type in a context that requires 
// an exact size

// When you have a large amount of data, 
// and you want to transfer ownership but ensure that 
// the data won’t be copied when you do so

// When you want to own a value, 
// and you care only that it’s a type that implements a 
// particular trait rather than being of a specific type

// To improve performance in this situation, 
// we can store the large amount of data on the 
// heap in a box. Then, only the small amount 
// of pointer data is copied around on the stack, 
// while the data it references stays in one place 
// on the heap. 

// Example

// Storing Data on the Heap
/*
fn main() {
    let b = Box::new(5);
    println!("b = {b}");
}
*/

// Multiple values in one Box (tuple or array)
/*
fn main() {
let b = Box::new((1, 2, 3));        // tuple
let arr = Box::new([10, 20, 30]);   // fixed-size array
println!("{:?}", b.2); 
println!("{:?}", arr[2]);             // prints 2
}
*/


// Dynamic collection of values (Vec inside a Box)
fn main() {
let mut b = Box::new(vec![1, 2, 3, 4, 5]);
b.push(6);
b.push(7);
b.push(8);
 // works because Box derefs to Vec
println!("{:?}", b);
}





