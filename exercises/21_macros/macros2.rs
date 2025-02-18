/*
    Hint
    Macros don't quite play by the same rules as the rest of Rust, in terms of
    what's available where.

    Unlike other things in Rust, the order of "where you define a macro" versus
    "where you use it" actually matters.
*/

// TODO: Fix the compiler error by moving the whole definition of this macro.
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}
