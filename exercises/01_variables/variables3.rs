/*
    Hint
    In this exercise, we have a variable binding that we've created in the `main`
    function, and we're trying to use it in the next line, but we haven't given it
    a value.

    We can't print out something that isn't there; try giving `x` a value!

    This is an error that can cause bugs that's very easy to make in any
    programming language -- thankfully the Rust compiler has caught this for us!
*/

fn main() {
    // TODO: Change the line below to fix the compiler error.
    let x: i32 = 10;

    println!("Number {x}");
}
