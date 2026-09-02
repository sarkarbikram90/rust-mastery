//Today we have learned about if else conditionals in Rust.

fn main() {
    let switch = "on";
    
if switch == "on" {
        println!("The switch is on");
    } else if switch == "off" {
        println!("The switch is off");
    } else {
        println!("The switch is in an unknown state");
    }
}

