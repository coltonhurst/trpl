/*

Ownership in Rust
https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

1. Each value in Rust has a variable that's called its owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

*/

fn main() {
    // ------------------------------ Brief notes on ownership:

    let s = "hello";    // s owns hello, & it goes out of scope when this function ends

    println!("{}", s);

    let mut st = String::from("hello");     // the second string type, allocated on the heap, can be mutated

    st.push_str(", world!");

    println!("{}", st);

    let s1 = String::from("hello");
    let s2 = s1;    // "move". s1 is a pointer to the data on the heap,
                    // and we point s2 to the same loc in the heap

    //println!("{}, world!", s1);
    // The above line will fail. This is because when s1 and s2 go out of scope,
    // they will both try to free the same memory. Known as a "double gree" error.
    // So, Rust considers s1 to no longer be valid after the move- therefore, you can't print it.
    // Remember, data can only have ONE owner.
    // Rust will never automatically create "deep" copies of data- that must be done explicitly.

    // see chapter 10 & "Derivable Traits" in Appendix C for copy trait explanations

    // ------------------------------ Ownership with functions:

    let s3 = String::from("hello");     // s comes into scope

    takes_ownership(s3);     // s3's value moves into the function...
                            // ... and so is no longer valid here

    let x = 5;              // x comes into scope

    makes_copy(x);          // x would move into the function,
                            // but i32 is Copy, so it's okay to still use x afterwards

    // ------------------------------ Giving ownership:
    let a1 = gives_ownership();

    let a2 = String::from("hello");

    let a3 = takes_and_gives_back(a2);

    // Returning a tuple
    let m = String::from("hello");

    let (y, z) = calculate_length(m);

    println!("The length of '{}' is {}.", y, z);  // to print out m, our string, after
                                            // it has lost ownership, we need to use a tuple
                                            // to give the ownership to y

}   // Here, x goes out of scope, then s3. But because s3's value was moved, nothing special happens.

fn takes_ownership(some_string: String) {   // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and 'drop' is called. The backing memory is freed.

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    let some_string = String::from("hello");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}