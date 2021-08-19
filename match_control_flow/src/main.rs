fn main() {
    let coin = Coin::Penny;
    println!("{:?} is {}", coin, vlaue_in_cents(&coin));
    println!("{}", vlaue_in_cents(&Coin::Dime));
    println!("{}", vlaue_in_cents(&Coin::Nickel));

    let coin = Coin::Quarter(UsState::Alaska);
    println!("{:?} is {}", coin, vlaue_in_cents(&coin));
    println!("{}", vlaue_in_cents(&Coin::Quarter(UsState::Alabam)));
    println!("{}", vlaue_in_cents(&Coin::Quarter(UsState::California)));

    println!("{} plus one is {:?}", 1, plus_one(Some(1)));
    println!("None plus one is {:?}", plus_one(None));

    for i in 0..10 {
        match_palceholder(i);
    }
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn vlaue_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 0,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("quater coin from {:?}", state);
            25
        },
    }
}

#[derive(Debug)]
enum UsState{
    Alabam,
    Alaska,
    California
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(x) => Some(x + 1),
    }
}

fn match_palceholder(value : u8)  {
    match value {
        0 => println!("0"),
        1 => println!("1"),
        2 => println!("2"),
        _ => println!("{}", value)
    }
}