struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
fn main() {
    let user1 = User {
        email: String::from("email@gmail.com"),
        username: String::from("username"),
        active: true,
        sign_in_count: 1,
    };

    println!("The username is {}", user1.username);

    // using the struct update syntax
    let user2 = User {
        email: String::from("anotheremail@gmail.com"),
        ..user1
    };

    println!("The other username is {}", user2.username);

    /*
     * Tuple structs
     *
     */

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}