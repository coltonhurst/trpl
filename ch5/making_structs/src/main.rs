// Creating structs in Rust

struct Point(i32, i32, i32);    // a tuple struct

fn main() {
    let user1 = User {
        email: String::from("joebro@test.com"),
        username: String::from("joebro"),
        sign_in_count: 1,
        active: true,
    };

    println!("{:?}", user1);
    println!("{:?}", build_user(String::from("happybday@hi.com"), String::from("hbday")));

    let origin = Point(0, 0, 0);

    let user2 = User {
        email: String::from("joebro2@test.com"),
        username: String::from("joebro2"),
        ..user1     // use user1 data to populate the rest of the required fields that aren't explicitly mentioned here
    };

    println!("{:?}", user2);
}

#[derive(Debug)]    // a trait, lets me print the struct
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// A function that returns a user
// Notice we used shorthand as the struct field name and parameter names match
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}