fn main() {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}"); // this will print 12 because the inner scope shadows the outer x
    }

    println!("The value of x is: {x}");
}
