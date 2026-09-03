use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // Step 1: Trim first and store in a variable
    let trimmed = guess.trim();

    // Step 2: Check for leading '+' BEFORE parsing
    if trimmed.starts_with('+') {
        // ✅ Error → stderr
        eprintln!("Please do not use a leading '+' sign"); 
        return;
    }

    // Step 3: Parse AFTER the check
    let guess_number: u32 = trimmed
        .parse()
        .expect("Please type a valid number");

    // ✅ Success → stdout
    println!("You guessed: {guess_number}");
}