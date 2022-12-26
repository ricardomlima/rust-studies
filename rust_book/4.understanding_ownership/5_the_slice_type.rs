/**
 * Just like in Python :D
 */

fn main() {
    let input_word = String::from("Hello World");

    let word = first_word(&input_word);
    println!("{}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}