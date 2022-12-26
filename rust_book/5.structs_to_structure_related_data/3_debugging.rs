#[derive(Debug)]
struct Dog {
    name: String,
    age: u32,
}

fn main() {
    let jobim = Dog {
        name: String::from("Jobim"),
        age: 2,
    };

    println!("here he is: {:?}", jobim);

    println!("here he is prettier: {:#?}", jobim);

    let scale = 2;
    let kiara = Dog {
        name: String::from("Kiara"),
        age: dbg!(2 * scale),
    };

    dbg!(&kiara);
}