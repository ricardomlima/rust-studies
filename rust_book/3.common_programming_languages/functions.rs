/* FUNCTIONS

Names are snake case!

*/

fn main() {
    println!("main function");
    another_function();
    another_function_parameters(15, 'm');

    /*
     Statements do not return value.
     E.g:
    */
    let me = "test"; // this does not return a value. Not even the assignment value.

    /*
     because statemements don't return values we can't do this like in other languages:
     let x = y = 6;
    */

    /*
     Expressions return values! Expressions can be part of statements.
     Expressions do not include ending semicolons. If you add an ending
     semicolon you turn the expression into a statement and then it stops
     returning
    */
    let y_block = {
        let x = 3;
        x + 1
    };
    println!("The value of y_block is: {y_block}");

    let five_plus_one: i32 = five_plus(1);
    println!("the value of five_plus_one is {five_plus_one}");
}

fn another_function() {
    println!("Another function");
}

// in functions parameters must be declared with types!
fn another_function_parameters(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}")
}

// function with return type
fn five_plus(x: i32) -> i32 {
    5 + x
}