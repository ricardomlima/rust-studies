/* FUNCTIONS

Names are snake case!
*/

fn main() {
    println!("main function");
    another_function();
    another_function_parameters(15);
}

fn another_function() {
    println!("Another function");
}

fn another_function_parameters(x: i32) {
    println!("The value of x is {x}")
}