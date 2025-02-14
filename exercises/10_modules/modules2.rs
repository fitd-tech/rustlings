// You can bring module paths into scopes and provide new names for them with
// the `use` and `as` keywords.

/*
    Hint
    The `delicious_snacks` module is trying to present an external interface that
    is different than its internal structure (the `fruits` and `veggies` modules
    and associated constants). Complete the `use` statements to fit the uses in
    `main` and find the one keyword missing for both constants.

    Learn more in The Book:
    https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html#re-exporting-names-with-pub-use
*/

mod delicious_snacks {
    // TODO: Add the following two `use` statements after fixing them.
    // use self::fruits::PEAR as ???;
    // use self::veggies::CUCUMBER as ???;
    pub use self::fruits::PEAR as fruit;
    pub use self::veggies::CUCUMBER as veggie;

    mod fruits {
        pub const PEAR: &str = "Pear";
        pub const APPLE: &str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &str = "Cucumber";
        pub const CARROT: &str = "Carrot";
    }
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie,
    );
}
