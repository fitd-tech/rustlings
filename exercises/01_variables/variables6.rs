// TODO: Change the line below to fix the compiler error.

/*
    Hint
    We know about variables and mutability, but there is another important type of
    variable available: constants.

    Constants are always immutable. They are declared with the keyword `const`
    instead of `let`.

    The type of Constants must always be annotated.

    Read more about constants and the differences between variables and constants
    under 'Constants' in the book's section 'Variables and Mutability':
    https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#constants
*/

const NUMBER: i32 = 3;

fn main() {
    println!("Number: {NUMBER}");
}
