// important Rust concepts
// Enums are extremely powerful in Rust
// enum creates a real domain type

enum BreakerState {
    Closed,
    Open,
    HalfOpen,
}

// Rust enums are not just integer constants.
// They are a proper type.
// The compiler can therefore help 
// ensure you're working with legitimate 
// states instead of arbitrary strings

// "closed"
// "OPEN"
// "half-open"  // These are just strings. 

// three lets
// demonstrate shadowing

fn check_state(state: &BreakerState) {
    // match is Rust's main
    // pattern-matching
    // mechanism
    // Rust requires match to be exhaustive
    // which means you have to handle every possible case
    match state {
        BreakerState::Closed => println!("The circuit is closed."),
        BreakerState::Open => println!("The circuit is open."),
        BreakerState::HalfOpen => println!("The circuit is half open."),
    }
}

// basically says

// basically says:

// Given a BreakerState:
// Closed   → do this
// Open     → do this
// HalfOpen → do this

// type safety
// applied to domain modeling

fn main() {
    let mut breaker_state = BreakerState::Closed;

    check_state(&breaker_state);

    breaker_state = BreakerState::Open;
    check_state(&breaker_state);

    breaker_state = BreakerState::HalfOpen;
    check_state(&breaker_state);
}

// If you later add
// enum BreakerState {
//     Closed,
//     Open,
//     HalfOpen,
//     Disabled,
// }
// Suddenly, every function calling check_state
// will fail to compile.
// This is GOOD. It forces you to handle the new case.

// If you use strings instead:
// let state = "InvalidState";
// println!("State: {}", state);  // No error!
// Rust prevents runtime bugs at compile time.
// This is Rust’s whole philosophy.

// your existing match will fail to compile until
// you handle Disabled. 
// This forces you to
// think about all possible states.

// for production systems
// A state machine gains a new state
// → compiler tells you everywhere
// you haven't handled it.

// This is not just about code safety.
// It is about 
// expressing your domain correctly.
