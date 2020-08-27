// A first word program

fn main() {
    let words = String::from("bacon apple pasta watermelon orange blueberry");

    println!("The first word is: {}", first_word(&words));
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]
        }
    }

    return &s[..]
}