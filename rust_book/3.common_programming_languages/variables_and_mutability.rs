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

    // shadowing
    let z = 5;

    // z has been shadowed
    let z = z + 1;

    {
        // inner scope borrowing the value from the outer scope
        // but not shadowing the outer scope
        let z = z * 2;
        println!("The value of z in the inner scope is: {z}");
    }

    // inner scopes do not shadow outer scopes
    println!("The value of z is: {z}");
}