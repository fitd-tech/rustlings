// TODO: Add the missing type of the argument `num` after the colon `:`.

/*
    Hint
    Rust requires that all parts of a function's signature have type annotations,
    but `call_me` is missing the type annotation of `num`.
*/

fn call_me(num: i32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    call_me(3);
}
