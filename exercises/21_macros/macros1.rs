/*
    Hint
    When you call a macro, you need to add something special compared to a regular
    function call.
*/

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    // TODO: Fix the macro call.
    my_macro!();
}
