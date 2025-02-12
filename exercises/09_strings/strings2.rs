// TODO: Fix the compiler error in the `main` function without changing this function.

/*
    Hint
    Yes, it would be really easy to fix this by just changing the value bound to
    `word` to be a string slice instead of a `String`, wouldn't it? There is a way
    to add one character to the `if` statement, though, that will coerce the
    `String` into a string slice.

    Side note: If you're interested in learning about how this kind of reference
    conversion works, you can jump ahead in the book and read this part in the
    smart pointers chapter:
    https://doc.rust-lang.org/book/ch15-02-deref.html#implicit-deref-coercions-with-functions-and-methods
*/

fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}

fn main() {
    let word = String::from("green"); // Don't change this line.

    if is_a_color_word(&word) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}
