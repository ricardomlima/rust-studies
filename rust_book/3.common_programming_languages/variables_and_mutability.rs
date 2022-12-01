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

}