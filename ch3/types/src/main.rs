/*
    Rust Data Types

    Scalar
      - Integer
      - Floating-Point
      - Boolean
      - Character

    Compound
      - Tuple
      - Array
*/

fn main() {

    /* Scalar type examples */

    // two floating point variables
    let x = 2.0;        // f64
    let y: f32 = 3.0;   // f32
    println!("x = {} and y = {}", x, y);

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    // booleans
    let t = true;
    let f: bool = false;    // with explicit type annotation

    // char (is 4 bytes, represents a Unicode Scalar Value)
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    /* Compound type examples */

    // tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // destructuing a tuple to get the values (can also use .)
    let (x, y, z) = tup;
    println!("The value of y is {}", y);
    println!("The value of x is {}", tup.0);

    // arrays (allocated on the stack)
    let a = [1, 2, 3, 4, 5];

    let months = ["January", "February", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"];

    // varName: [type, # of elements]
    let a: [i32; 5] = [6, 7, 8, 9, 10];

    // creates an array of 5 elements, with all elements set to the value 3 initially
    let a = [3; 5];

    // access array elements using indexing (as normal)
    println!("The first element of a is {}", a[0]);

    let element = 10;

    // Witness Rust protecting us with a runtime error
    println!("Let's try to access out of bounds memory and see if Rust handles it: {}", a[element]);
}