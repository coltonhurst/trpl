/*
    Functions

    Statements vs Expressions

    // Uh... this is a comment!
*/

fn main() {
    println!("Hello, world!");

    // functions & func parameter examples
    another_function();
    another_function2(5);
    another_function3(5, 10);

    // statement
    let x = 6;

    // combining statements & expressions
    let y = {
        let x = 3;
        x + 1       // expressions don't have ;
    };
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

    // returning a value from a function
    let x = five();
    println!("The value of x is: {}", x);

    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

fn another_function() {
    println!("Another function.");
}

fn another_function2(x: i32) {
    println!("The value of x is: {}", x);
}

fn another_function3(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}