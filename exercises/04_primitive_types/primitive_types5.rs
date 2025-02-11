/*
    Hint
    Take a look at the 'Data Types -> The Tuple Type' section of the book:
    https://doc.rust-lang.org/book/ch03-02-data-types.html#the-tuple-type
    Particularly the part about destructuring (second to last example in the
    section).

    You'll need to make a pattern to bind `name` and `age` to the appropriate parts
    of the tuple.
*/

fn main() {
    let cat = ("Furry McFurson", 3.5);

    // TODO: Destructure the `cat` tuple in one statement so that the println works.
    // let /* your pattern here */ = cat;
    let (name, age) = cat;

    println!("{name} is {age} years old");
}
