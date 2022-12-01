fn main() {
    // variables are by default immutable
    let x = 5;
    println!("The value of x is: {x}");

    // mut indicates that a variable is mutable
    let mut y = 10;
    println!("The value of y is: {y}");

    // now you can change the mutable variable
    y = 25;
    println!("The value of y is: {y}");

    // constants cannot be changed and must be in uppercase by convention
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
}