/*
    Hint
    The compiler message is saying that Rust can't infer the type that the
    variable binding `x` has with what is given here.

    What happens if you annotate the first line in the `main` function with a type
    annotation?

    What if you give `x` a value?

    What if you do both?

    What type should `x` be, anyway?

    What if `x` is the same type as `10`? What if it's a different type?
*/

fn main() {
    // TODO: Change the line below to fix the compiler error.
    let mut x = 42;

    if x == 10 {
        println!("x is ten!");
    } else {
        println!("x is not ten!");
    }

    x = 10;

    if x == 10 {
        println!("x is ten!");
    } else {
        println!("x is not ten!");
    }
}
