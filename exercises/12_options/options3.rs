/*
    Hint
    The compiler says a partial move happened in the `match` statement. How can
    this be avoided? The compiler shows the correction needed.

    After making the correction as suggested by the compiler, read the related docs
    page:
    https://doc.rust-lang.org/std/keyword.ref.html
*/

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let optional_point = Some(Point { x: 100, y: 200 });

    // TODO: Fix the compiler error by adding something to this match statement.
    match optional_point {
        Some(ref p) => println!("Co-ordinates are {},{}", p.x, p.y),
        _ => panic!("No match!"),
    }

    println!("{optional_point:?}"); // Don't change this line.
}
