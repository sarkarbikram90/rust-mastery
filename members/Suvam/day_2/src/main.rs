//Learned About 1) Cargo Build 2) Cargo Run 3) Cargo Check 4) Cargo Clean
//Learned about 1) Shadowing 2) Mutability
//Learned about 1) Variable Binding 


fn main () {
    let mut temp = 20;
    let temp = 30;
    let temp = 40;
    println! ("The Temperature now: {temp}");
}

//I never mutate binding #1. I was  creating new bindings that shadow it.
//that's why Rust was giving me a warning. " warning: variable does not need to be mutable "

// corrected version

fn main () {
    let mut temp = 20;
    println!("The Temperature now: {temp}");
    
    temp = 30;
    println! ("The Themperature now: {temp}");

    temp = 40;
    println! ("The Temperature now: {temp}");
}
