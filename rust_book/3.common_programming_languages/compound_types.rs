fn main() {
    /* TUPLE ARRAY

    can store different types and must have fixed length
    */
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // retrieving elements from a tuple
    let (x, y, z) = tup;

    println!("The value of y is {y}");

    //retrieving elements from a tuple from its index
    let zed = tup.2;

    println!("The value of z is {zed}");

    /* ARRAY

    can only store same types and must have fixed length
    */
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    println!("The value of the first element of the array is {first}");
}