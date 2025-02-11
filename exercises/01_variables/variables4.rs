// TODO: Fix the compiler error.

/*
    Hint
    In Rust, variable bindings are immutable by default. But here, we're trying
    to reassign a different value to `x`! There's a keyword we can use to make
    a variable binding mutable instead.
*/

fn main() {
    let mut x = 3;
    println!("Number {x}");

    x = 5; // Don't change this line
    println!("Number {x}");
}
