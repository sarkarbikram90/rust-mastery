use day_14::{Coin, UsState};

fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => println!("You rolled a three!"),
        7 => println!("You rolled a seven!"),
        other => println!("You rolled a {}!", other),
    }

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    let coin = Coin::Quarter(UsState::Alabama);
    if let Some(description) = describe_state_quarter(coin) {
        println!("{description}");
    }
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    } else {
        None
    }
}
