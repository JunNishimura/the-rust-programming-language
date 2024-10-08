enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    let coin = Coin::Penny;
    let value = value_in_cents(coin);
    println!("Value in cents: {}", value);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("Six: {:?}", six);
    println!("None: {:?}", none);

    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("Three!");
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}