use crate::dictionary::{SignedCounter, UnsignedCounter};

pub fn next_signed(counter: SignedCounter) -> SignedCounter {
    counter + 1
}

pub fn next_unsigned(counter: UnsignedCounter) -> UnsignedCounter {
    counter + 1
}

pub fn prev_signed(counter: SignedCounter) -> SignedCounter {
    counter - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_signed() {
        assert_eq!(next_signed(1), 2);
    }

    #[test]
    fn test_next_unsigned() {
        assert_eq!(next_unsigned(1), 2);
    }

    #[test]
    fn test_prev_signed() {
        assert_eq!(prev_signed(1), 0);
    }
}
