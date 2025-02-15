/*
    Hint
    `Ok` and `Err` are the two variants of `Result`, so what the tests are saying
    is that `generate_nametag_text` should return a `Result` instead of an `Option`.

    To make this change, you'll need to:
    - update the return type in the function signature to be a `Result<String,
        String>` that could be the variants `Ok(String)` and `Err(String)`
    - change the body of the function to return `Ok(…)` where it currently
        returns `Some(…)`
    - change the body of the function to return `Err(error message)` where it
        currently returns `None`
*/

// TODO: This function refuses to generate text to be printed on a nametag if
// you pass it an empty string. It'd be nicer if it explained what the problem
// was instead of just returning `None`. Thankfully, Rust has a similar
// construct to `Option` that can be used to express error conditions. Change
// the function signature and body to return `Result<String, String>` instead
// of `Option<String>`.
fn generate_nametag_text(name: String) -> Result<String, String> {
    if name.is_empty() {
        // Empty names aren't allowed
        Err(String::from("Empty names aren't allowed"))
    } else {
        Ok(format!("Hi! My name is {name}"))
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".to_string()).as_deref(),
            Ok("Hi! My name is Beyoncé"),
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text(String::new())
                .as_ref()
                .map_err(|e| e.as_str()),
            Err("Empty names aren't allowed"),
        );
    }
}
