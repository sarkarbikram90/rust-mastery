use std::io;

fn main() {
    println!("Enter a number:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let number: u64 = input.trim().parse().expect("Please enter a whole number");

    if is_prime(number) {
        println!("{number} is prime.");
    } else {
        println!("{number} is not prime.");
    }
}

fn is_prime(number: u64) -> bool {
    if number < 2 {
        return false;
    }

    let mut divisor = 2;
    while divisor <= number / divisor {
        if number % divisor == 0 {
            return false;
        }
        divisor += 1;
    }

    true
}
