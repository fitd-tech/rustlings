/*
    Hint
    The difference between this one and the previous ones is that the first line
    of `fn fill_vec` that had `let mut vec = vec;` is no longer there. You can,
    instead of adding that line back, add `mut` in one place that will change
    an existing binding to be a mutable binding instead of an immutable one :)
*/

// TODO: Fix the compiler error in the function without adding any new line.
fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(88);

    vec
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics3() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}
