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
}