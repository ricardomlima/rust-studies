fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);

    let dice_roll = 9;

    match dice_roll {
        3 => {
            println!("You won a hat!");
        }
        7 => {
            println!("You won some cash");
        }
        other => {
            // you can use _ instead of other in case you won't use the variable!
            println!("you move {}", other);
        }
    }
}