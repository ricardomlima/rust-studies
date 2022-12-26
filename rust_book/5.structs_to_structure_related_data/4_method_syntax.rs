#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other_rectangle: &Rectangle) -> bool {
        self.width > other_rectangle.width && self.height > other_rectangle.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 15,
        height: 25,
    };

    let rect3 = Rectangle {
        width: 90,
        height: 100,
    };
    println!("The area of the rectangle is {}", rect1.area());

    println!("it is {} that rectangle 1 can hold rectangle 2", rect1.can_hold(&rect2));

    println!("it is {} that rectangle 1 can hold rectangle 3", rect1.can_hold(&rect3));

    let square = Rectangle::square(4);

    println!("The area of the square is {}", square.area());
}