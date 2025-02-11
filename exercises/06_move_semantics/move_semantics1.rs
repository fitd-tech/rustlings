// TODO: Fix the compiler error in this function.

/*
    Hint
    So you've got the "cannot borrow `vec` as mutable, as it is not declared as
    mutable" error on the line where we push an element to the vector, right?

    The fix for this is going to be adding one keyword, and the addition is NOT on
    the line where we push to the vector (where the error is).

    Try accessing `vec0` after having called `fill_vec()`. See what happens!
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

    #[test]
    fn move_semantics1() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0);
        // println!("vec0 {:?}", vec0);
        assert_eq!(vec1, vec![22, 44, 66, 88]);
    }
}
