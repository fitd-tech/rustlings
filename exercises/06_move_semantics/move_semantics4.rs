/*
    Hint
    Carefully reason about the range in which each mutable reference is in
    scope. Does it help to update the value of `x` immediately after
    the mutable reference is taken?
    Read more about 'Mutable References' in the book's section 'References and
    Borrowing':
    https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#mutable-references.
*/

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // TODO: Fix the compiler errors only by reordering the lines in the test.
    // Don't add, change or remove any line.
    #[test]
    fn move_semantics4() {
        let mut x = Vec::new();
        let y = &mut x;
        y.push(42);
        let z = &mut x;
        z.push(13);
        assert_eq!(x, [42, 13]);
    }
}
