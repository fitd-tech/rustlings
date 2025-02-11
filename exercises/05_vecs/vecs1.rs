/*
    Hint
    In Rust, there are two ways to define a Vector.
    1. One way is to use the `Vec::new()` function to create a new vector
    and fill it with the `push()` method.
    2. The second way is to use the `vec![]` macro and define your elements
    inside the square brackets. This way is simpler when you exactly know
    the initial values.

    Check this chapter: https://doc.rust-lang.org/book/ch08-01-vectors.html
    of the Rust book to learn more.
*/

fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // Array

    // TODO: Create a vector called `v` which contains the exact same elements as in the array `a`.
    // Use the vector macro.
    // let v = ???;
    let v = vec![10, 20, 30, 40];

    (a, v)
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, *v);
    }
}
