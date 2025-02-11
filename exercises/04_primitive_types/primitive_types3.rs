/*
    Hint
    There's a shorthand to initialize arrays with a certain size that doesn't
    require you to type in 100 items (but you certainly can if you want!).

    For example, you can do:
    ```
    let array = ["Are we there yet?"; 100];
    ```

    Bonus: what are some other things you could have that would return `true`
    for `a.len() >= 100`?
*/

fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    // let a = ???
    let a = [0; 100];

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
