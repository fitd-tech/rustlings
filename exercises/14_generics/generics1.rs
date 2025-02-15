// `Vec<T>` is generic over the type `T`. In most cases, the compiler is able to
// infer `T`, for example after pushing a value with a concrete type to the vector.
// But in this exercise, the compiler needs some help through a type annotation.

/*
    Hint
    Vectors in Rust make use of generics to create dynamically sized arrays of any
    type.
    If the vector `numbers` has the type `Vec<T>`, then we can only push values of
    type `T` to it. By using `into()` before pushing, we ask the compiler to convert
    `n1` and `n2` to `T`. But the compiler doesn't know what `T` is yet and needs a
    type annotation.

    `u8` and `i8` can both be converted to `i16`, `i32` and `i64`. Choose one for
    the generic of the vector.
*/

fn main() {
    // TODO: Fix the compiler error by annotating the type of the vector
    // `Vec<T>`. Choose `T` as some integer type that can be created from
    // `u8` and `i8`.
    let mut numbers: Vec<i16> = Vec::new();

    // Don't change the lines below.
    let n1: u8 = 42;
    numbers.push(n1.into());
    let n2: i8 = -1;
    numbers.push(n2.into());

    println!("{numbers:?}");
}
