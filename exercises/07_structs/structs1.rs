/*
    Hint
    Rust has more than one type of struct. Three actually, all variants are used to
    package related data together.

    There are regular structs. These are named collections of related data stored in
    fields.

    Tuple structs are basically just named tuples.

    Finally, unit structs. These don't have any fields and are useful for generics.

    In this exercise, you need to complete and implement one of each kind.
    Read more about structs in The Book:
    https://doc.rust-lang.org/book/ch05-01-defining-structs.html
*/

struct ColorRegularStruct {
    // TODO: Add the fields that the test `regular_structs` expects.
    // What types should the fields have? What are the minimum and maximum values for RGB colors?
    red: u8,
    green: u8,
    blue: u8,
}

struct ColorTupleStruct(
    /* TODO: Add the fields that the test `tuple_structs` expects */
    u8,
    u8,
    u8,
);

#[derive(Debug)]
struct UnitStruct;

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regular_structs() {
        // TODO: Instantiate a regular struct.
        // let green =
        let green = ColorRegularStruct {
            red: 0,
            green: 255,
            blue: 0,
        };

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct.
        // let green =
        let green = ColorTupleStruct(0, 255, 0);

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit struct.
        // let unit_struct =
        let unit_struct = UnitStruct;

        let message = format!("{unit_struct:?}s are fun!");

        assert_eq!(message, "UnitStructs are fun!");
    }
}
