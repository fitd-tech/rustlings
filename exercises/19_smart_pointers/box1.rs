// At compile time, Rust needs to know how much space a type takes up. This
// becomes problematic for recursive types, where a value can have as part of
// itself another value of the same type. To get around the issue, we can use a
// `Box` - a smart pointer used to store data on the heap, which also allows us
// to wrap a recursive type.
//
// The recursive type we're implementing in this exercise is the "cons list", a
// data structure frequently found in functional programming languages. Each
// item in a cons list contains two elements: The value of the current item and
// the next item. The last item is a value called `Nil`.

/*
    Hint
    The compiler's message should help: Since we cannot store the value of the
    actual type when working with recursive types, we need to store a reference
    (pointer) to its value.

    We should, therefore, place our `List` inside a `Box`. More details in The Book:
    https://doc.rust-lang.org/book/ch15-01-box.html#enabling-recursive-types-with-boxes

    Creating an empty list should be fairly straightforward (Hint: Read the tests).

    For a non-empty list, keep in mind that we want to use our `Cons` list builder.
    Although the current list is one of integers (`i32`), feel free to change the
    definition and try other types!
*/

// TODO: Use a `Box` in the enum definition to make the code compile.
#[derive(PartialEq, Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

// TODO: Create an empty cons list.
fn create_empty_list() -> List {
    List::Nil
}

// TODO: Create a non-empty cons list.
fn create_non_empty_list() -> List {
    List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))))
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(create_empty_list(), List::Nil);
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list());
    }
}
