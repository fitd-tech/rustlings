/*
    Hint
    When running this exercise for the first time, you'll notice an error about
    "borrow of moved value". In Rust, when an argument is passed to a function and
    it's not explicitly returned, you can't use the original variable anymore.
    We call this "moving" a variable. When we pass `vec0` into `fill_vec`, it's
    being "moved" into `vec1`, meaning we can't access `vec0` anymore.

    You could make another, separate version of the data that's in `vec0` and
    pass it to `fill_vec` instead. This is called cloning in Rust.
*/

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(88);

    vec
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Make both vectors `vec0` and `vec1` accessible at the same time to
    // fix the compiler error in the test.
    #[test]
    fn move_semantics2() {
        let vec0 = vec![22, 44, 66];

        let vec0_copy = vec0.clone();
        let vec1 = fill_vec(vec0_copy);

        assert_eq!(vec0, [22, 44, 66]);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}
