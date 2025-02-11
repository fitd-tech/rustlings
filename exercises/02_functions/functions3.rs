/*
    Hint
    This time, the function *declaration* is okay, but there's something wrong
    with the place where we are calling the function.
*/

fn call_me(num: u8) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    // TODO: Fix the function call.
    call_me(3);
}
