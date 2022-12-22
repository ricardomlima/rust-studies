fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem, just reading an immutable reference
    let r2 = &s; // no problem, just reading an immutable reference

    println!("{} {}", r1, r2); // no problem they won't change

    let r3 = &mut s; // here we can make a mutable reference because the immutable references
    // have already been used. No risk of changing.
    // Users of an immutable reference don’t expect the value to suddenly change out from under them!
    println!("{}", r3);

    let reference_to_nothing = dangle();

    let reference_to_something = no_dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");
    &s // returns reference to a string that is dropped! Because this function owns it.
}

fn no_dangle() -> String {
    let s = String::from("hello");
    s // returns the string and moves the ownership to the caller.
    //No risk of it being dropped before being used outside!
}