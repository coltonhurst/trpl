fn main() {
    // mutable variables
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // The old x is "shadowed" by the new x (happens 2x)
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is {}", x);

    // A constant
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is {}", MAX_POINTS);

    // "shadowing" is basically just overwriting the variable, so you can have a new type
    // (can't do that with 'mut')
    let spaces = "    ";
    let spaces = spaces.len();
    println!("There are {} spaces!", spaces);
}
