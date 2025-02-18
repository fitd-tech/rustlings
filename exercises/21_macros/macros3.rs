/*
    Hint
    In order to use a macro outside of its module, you need to do something
    special to the module to lift the macro out into its parent.
*/

// TODO: Fix the compiler error without taking the macro definition out of this
// module.
#[macro_use]
mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}
