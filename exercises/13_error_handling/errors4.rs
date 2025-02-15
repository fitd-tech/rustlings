use std::cmp::Ordering;
/*
    Hint
    `PositiveNonzeroInteger::new` is always creating a new instance and returning
    an `Ok` result. But it should be doing some checking, returning an `Err` if
    those checks fail, and only returning an `Ok` if those checks determine that
    everything is… okay :)
*/

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<Self, CreationError> {
        // TODO: This function shouldn't always return an `Ok`.
        // Ok(Self(value as u64))

        // if value < 0 {
        //     Err(CreationError::Negative)
        // } else if value == 0 {
        //     Err(CreationError::Zero)
        // } else {
        //     Ok(Self(value as u64))
        // }

        match value.cmp(&0) {
            Ordering::Greater => Ok(Self(value as u64)),
            Ordering::Less => Err(CreationError::Negative),
            Ordering::Equal => Err(CreationError::Zero),
        }
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        assert_eq!(
            PositiveNonzeroInteger::new(10),
            Ok(PositiveNonzeroInteger(10)),
        );
        assert_eq!(
            PositiveNonzeroInteger::new(-10),
            Err(CreationError::Negative),
        );
        assert_eq!(PositiveNonzeroInteger::new(0), Err(CreationError::Zero));
    }
}
