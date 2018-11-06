//模式匹配

fn main() {
    let coin = value_in_cents(Coin::Penny);
    println!("{}", coin);

    let quarter = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("{}", quarter);

    let five = Some(5);
    let six = increment(five);

    match six {
        None => println!("None"),
        Some(val) => println!("six.value={}", val),
    }

    let one = increment(None);
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn increment(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}