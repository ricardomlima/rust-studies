fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem, just reading an immutable reference
    let r2 = &s; // no problem, just reading an immutable reference

    println!("{} {}", r1, r2); // no problem they won't change

    let r3 = &mut s; // here we can make a mutable reference because the immutable references
    // have already been used. No risk of changing.
    // Users of an immutable reference don’t expect the value to suddenly change out from under them!
    println!("{}", r3);
}