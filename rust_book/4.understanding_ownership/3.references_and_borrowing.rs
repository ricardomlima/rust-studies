fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}", s1, len);

    let mut s2 = String::from("hi"); // we are going to change this so it needs to be mut

    change_it(&mut s); //OBS: YOU CAN ONLY HAVE ONE MUTABLE REFERENCE TO A VALUE PER SCOPE
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // s is not dropped since we only have it's reference and don't own it.

fn change_it(some_string: &mut String) {
    some_string.push_str(" people"); // this string we can change because we are referencing a mutable reference
}