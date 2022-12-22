fn main() {
    /*
    Since we are dealing with scalar types which are of fixed
    size in memory, rust actually copies using the Copy trait.
    This uses the memory stack. Copies are quick to make.

    -> Deep copy
    */
    let x = 5;
    let y = x;

    /*
    When dealing with complex types such as String rust allocates 
    on the stack a pointer to a heap. 
    Making a copy of a heap is very expensive.
    So by design: 
    Rust actually moves the pointer to be owned by s2 and invalidates s1.

    -> Not a shallow copy although it looks like it
        but since Rust invalidates the previous variable (s1) it's called
        a MOVE.
    */
    let s1 = String::from("hello");
    let s2 = s1;
}