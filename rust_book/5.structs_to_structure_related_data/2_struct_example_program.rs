struct Rectangle(u32, u32);

struct RectangleStruct {
    width: u32,
    height: u32,
}

fn main() {
    // Basic implementation
    let width1 = 30;
    let height1 = 30;
    println!("Area: {}", area(width1, height1));

    // Tuple implementation
    let rectangle_tuple = Rectangle(50, 30);
    println!("Area with tuple: {}", area_tuple(rectangle_tuple));

    // Struct implementation
    let rectangle_struct = RectangleStruct {
        width: 70,
        height: 90,
    };
    println!("Area with struct: {}", area_struct(&rectangle_struct));
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: Rectangle) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &RectangleStruct) -> u32 {
    rectangle.width * rectangle.height
}