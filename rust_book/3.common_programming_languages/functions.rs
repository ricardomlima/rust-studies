/* FUNCTIONS

Names are snake case!
*/

fn main() {
    println!("main function");
    another_function();
    another_function_parameters(15, 'm');
}

fn another_function() {
    println!("Another function");
}

// in functions parameters must be declared with types!
fn another_function_parameters(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}")
}