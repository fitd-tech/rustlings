/*
    Hint
    Take a look at the 'Understanding Ownership -> Slices -> Other Slices' section
    of the book: https://doc.rust-lang.org/book/ch04-03-slices.html and use the
    starting and ending (plus one) indices of the items in the array that you want
    to end up in the slice.

    If you're curious why the first argument of `assert_eq!` does not have an
    ampersand for a reference since the second argument is a reference, take a look
    at the coercion chapter of the nomicon:
    https://doc.rust-lang.org/nomicon/coercions.html
*/

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];

        // TODO: Get a slice called `nice_slice` out of the array `a` so that the test passes.
        // let nice_slice = ???
        let nice_slice = &a[1..4];

        assert_eq!([2, 3, 4], nice_slice);
    }
}
