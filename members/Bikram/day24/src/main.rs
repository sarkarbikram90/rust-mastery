// How to Write Tests
// Tests are Rust functions that 
// verify that the non-test code is functioning 
// in the expected manner. The bodies of test functions typically 
// perform these three actions:

// Set up any needed data or state.
// Run the code you want to test.
// Assert that the results are what you expect.

// Structuring Test Functions
// The file starts with an example add function so 
// that we have something to test.
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

fn main(){
    println!("Tests are great!");
}

// The cargo test command runs all tests in our project
// and displays output similar to this:
// refer to https://doc.rust-lang.org/book/ch11-01-writing-tests.html

