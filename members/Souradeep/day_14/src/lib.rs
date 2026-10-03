#[derive(Debug)]
pub enum UsState {
    Alabama,
    Alaska,
}

impl UsState {
    pub fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
        }
    }
}

pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
