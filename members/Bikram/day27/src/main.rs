// Fundamentals of Asynchronous Programming: 
// Async, Await, Futures, and Streams

// Asynchronous programming is a way to write 
// code that can perform multiple operations 
// concurrently, without blocking the main 
// thread of execution.

// Async
// Async is a keyword used to define an 
// asynchronous function.
// It is used to indicate that the function 
// will be executed asynchronously.

// example of aync

use tokio::time::sleep;
use std::time::Duration;

async fn say_hello_after_delay(name: &str, delay_seconds: u64) {
    // Simulate an asynchronous operation (like a network request or file I/O)
    // that would normally block the thread if done synchronously.
    sleep(Duration::from_secs(delay_seconds)).await;
    println!("Hello, {}! (after {} seconds)", name, delay_seconds);
}

#[tokio::main]
async fn main() {
    println!("Main function started.");

    // Launch two async tasks concurrently using tokio::spawn
    let task1 = tokio::spawn(say_hello_after_delay("Alice", 2));
    let task2 = tokio::spawn(say_hello_after_delay("Bob", 1));

    // Wait for both tasks to complete.
    // If we didn't await them here, the main function might finish
    // before the async tasks get a chance to run.
    let _ = tokio::join!(task1, task2);

    println!("Main function finished.");
}


