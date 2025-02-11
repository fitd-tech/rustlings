// Characters (`char`)

/*
    Hint
    No hints this time ;)
*/

fn main() {
    fn check_character_type(character: char) {
        println!("Character: {:?}", character);
        if character.is_alphabetic() {
            println!("Alphabetical!");
        } else if character.is_numeric() {
            println!("Numerical!");
        } else {
            println!("Neither alphabetic nor numeric!");
        }
    }
    // Note the _single_ quotes, these are different from the double quotes
    // you've been seeing around.
    let my_first_initial = 'C';
    check_character_type(my_first_initial);

    // TODO: Analogous to the example before, declare a variable called `your_character`
    // below with your favorite character.
    // Try a letter, try a digit (in single quotes), try a special character, try a character
    // from a different language than your own, try an emoji 😉
    // let your_character = '';
    let mut your_character = 't';
    check_character_type(your_character);
    your_character = '1';
    check_character_type(your_character);
    your_character = '£';
    check_character_type(your_character);
    your_character = '🫠';
    check_character_type(your_character);
}
