// Control Flow
/*
The ability to run some 
code depending on whether a 
condition is true and the ability 
to run some code repeatedly while 
a condition is true are basic 
building blocks in most programming languages. 
The most common constructs that let you control 
the flow of execution of Rust code are if expressions and loops.
*/

// if Expressions
// “If this condition is met, run this block of code. 
// If the condition is not met, do not run this block of code.”

/*
fn main() { // entry point
    let number = 6; // variable assignment

    if number < 5 { // condition
        // code to be executed if condition is true
        println!("condition was or is true"); 
    } // else { // else is optional
        // code to be executed if condition is false
        println!("condition was false"); 
    // }
}
*/

/*
fn main() {
    let number = 3; // lifetime starts here

    if number != 0 {
        println!("number was something other than zero");
    }
} // lifetime ends here

*/

fn main() {
    let door_open = true;
    let window_open = false;
    let alarm_armed = true;
    let motion_sensor = true;
    let valid_keycode = false;
    let time_since_trigger = 5; // seconds

    if !alarm_armed {
        println!("ALARM: Disarmed - Monitoring only");
    } else if valid_keycode && door_open {
        println!("ALARM: Disarmed via keypad entry");
    } else if door_open && !valid_keycode && time_since_trigger < 10 {
        println!("ALARM: ENTRY DELAY - Disarm now!");
    } else if window_open && motion_sensor {
        println!("ALARM: BURGLAR DETECTED - Window breach with motion!");
    } else if door_open && !valid_keycode {
        println!("ALARM: TRIGGERED - Unauthorized door entry!");
    } else if motion_sensor && !door_open && !window_open {
        println!("ALARM: MOTION DETECTED - Interior movement");
    } else if window_open && !motion_sensor {
        println!("ALARM: WINDOW OPEN - Possible break-in attempt");
    } else {
        println!("ALARM: All secure");
    }
}

