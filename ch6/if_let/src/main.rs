// Rust if let

fn main() {
    let some_u8_value = Some(0u8);

    // when we want to match on one value with match
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    // this behaves the same as above
    // (less boilerplate code)
    if let Some(3) = some_u8_value {
        println!("three");
    }
}
