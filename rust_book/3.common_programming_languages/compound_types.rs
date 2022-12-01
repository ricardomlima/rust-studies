fn main() {
    
    /* TUPLE ARRAY 
    
    can store different types
    */
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // retrieving elements from a tuple
    let (x, y, z) = tup;

    println!("The value of y is {y}");

    //retrieving elements from a tuple from its index
    let zed = tup.2;

    println!("The value of z is {zed}");

}
