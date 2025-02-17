/*
    Hint
    Remember that the generic lifetime `'a` will get the concrete lifetime that is
    equal to the smaller of the lifetimes of `x` and `y`.

    You can take at least two paths to achieve the desired result while keeping the
    inner block:
    1. Move the `string2` declaration to make it live as long as `string1` (how is
    `result` declared?)
    2. Move `println!` into the inner block
*/

// Don't change this function.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // TODO: Fix the compiler error by moving one line.

    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");
    let result;
    {
        result = longest(&string1, &string2);
    }
    println!("The longest string is '{result}'");
}
