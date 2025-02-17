// Tests are important to ensure that your code does what you think it should
// do.

/*
    Hint
    `assert!` is a macro that needs an argument. Depending on the value of the
    argument, `assert!` will do nothing (in which case the test will pass) or
    `assert!` will panic (in which case the test will fail).

    So try giving different values to `assert!` and see which ones compile, which
    ones pass, and which ones fail :)

    If you want to check for `false`, you can negate the result of what you're
    checking using `!`, like `assert!(!…)`.
*/

fn is_even(n: i64) -> bool {
    n % 2 == 0
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // TODO: Import `is_even`. You can use a wildcard to import everything in
    // the outer module.
    use super::*;

    #[test]
    fn you_can_assert() {
        // TODO: Test the function `is_even` with some values.
        assert!(is_even(8));
        assert!(!is_even(111));
    }
}
