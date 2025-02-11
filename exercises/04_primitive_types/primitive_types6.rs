/*
    Hint
    While you could use a destructuring `let` for the tuple here, try
    indexing into it instead, as explained in the last example of the
    'Data Types -> The Tuple Type' section of the book:
    https://doc.rust-lang.org/book/ch03-02-data-types.html#the-tuple-type
    Now, you have another tool in your toolbox!
*/

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn indexing_tuple() {
        let numbers = (1, 2, 3);

        // TODO: Use a tuple index to access the second element of `numbers`
        // and assign it to a variable called `second`.
        // let second = ???;
        let second = numbers.1;

        assert_eq!(second, 2, "This is not the 2nd number in the tuple!");
    }
}
