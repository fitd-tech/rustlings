// TODO: Add some function with the name `call_me` without arguments or a return value.

/*
    Hint
    This `main` function is calling a function that it expects to exist, but the
    function doesn't exist. It expects this function to have the name `call_me`.
    It also expects this function to not take any arguments and not return a value.
    Sounds a lot like `main`, doesn't it?
*/

fn main() {
    fn call_me() {
        return;
    }
    call_me(); // Don't change this line
}
