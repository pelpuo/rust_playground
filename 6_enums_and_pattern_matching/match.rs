#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

fn main() {
    let my_coin = Coin::Quarter(UsState::Alaska);
    let my_coin_value = value_in_cents(my_coin);

    println!("The value of my coin is {my_coin_value}");

}