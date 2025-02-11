/*
    Hint
    In `variables4` we already learned how to make an immutable variable mutable
    using a special keyword. Unfortunately this doesn't help us much in this
    exercise because we want to assign a different typed value to an existing
    variable. Sometimes you may also like to reuse existing variable names because
    you are just converting values to different types like in this exercise.

    Fortunately Rust has a powerful solution to this problem: 'Shadowing'!
    You can read more about 'Shadowing' in the book's section 'Variables and
    Mutability':
    https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#shadowing

    Try to solve this exercise afterwards using this technique.
*/

fn main() {
    let number = "T-H-R-E-E"; // Don't change this line
    println!("Spell a number: {}", number);

    // TODO: Fix the compiler error by changing the line below without renaming the variable.
    let number = 3;
    println!("Number plus two is: {}", number + 2);
}
