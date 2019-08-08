// Simple match operator examples

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter
}

fn main() {
    // coins
    let c: Coin = Coin::Penny;
    println!("A penny has a value of {} cent(s).", value_in_cents(c));

    // plus one examples
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("five is {}, six is {}, and none is {}.", get_value(five), get_value(six), get_value(none));
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

// Add one to the value wrapped in the Option<i32>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// Used to get the value wrapped in the Option<i32>
fn get_value(x: Option<i32>) -> i32 {
    match x {
        None => 0,
        Some(i) => i,
    }
}