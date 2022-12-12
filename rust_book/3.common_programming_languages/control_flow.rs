fn main() {
    let number: i32 = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("is divisible by 4");
    } else if number % 3 == 0 {
        println!("is divisible by 3");
    } else {
        println!("not divisible by 4 neither 3");
    }

    /*
        Using if in statements
    */
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("the value of number is: {number}");

    /*
        Repeating code with loop
    */
    let mut counter: isize = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            // here we return an expression with the break
            break counter * 2;
        }
    };
    println!("the result is {result}");
}