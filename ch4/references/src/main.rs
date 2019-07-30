/*
    References & Borrowing

    Recap:
    - "At any given time, you can have either one mutable reference or any number of immutable references."
    - "References must always be valid."
*/

fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);    // because this is a references, we are not passing ownership
                                        // this is called "borrowing"
    println!("The length of '{}' is {}.", s1, len);

    // a mutable reference
    // you can only borrow this once at time
    let mut s = String::from("hello");

    change(&mut s);

    println!("{}", s);

    // dangling references
    let reference_to_nothing = dangle();
}

fn calculate_length(s: &String) -> usize {  // s is a references to a String
    s.len()
}   // Here, s goes out of scope. But because it does not have ownership of what
    // it refers to, nothing happens.
    // Note: references are immutable by default.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}   // mutable reference
    // You can only have one mutable reference to a particular piece of data in a particular scope at a time

fn dangle() -> &String {    // dangle returns a reference to a string
    let s = String::from("hello");  // s is a new String

    &s  // we return a reference to the String, s
}   // Here, s goes out of scope, and is dropped. Its memory goes away.
    // Danger!

// The fix would be to return the string directly
// fn dangle() -> &String {