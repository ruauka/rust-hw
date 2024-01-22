pub type SignedCounter = isize;
pub type UnsignedCounter = usize;

#[derive(Debug)]
pub struct Counter {
    s_counter: SignedCounter,
    u_counter: UnsignedCounter,
}

impl Counter {
    pub fn new(s_counter: isize, u_counter: usize) -> Self {
        Self {
            s_counter,
            u_counter,
        }
    }

    pub fn next_signed(&self) -> SignedCounter {
        self.s_counter + 1
    }

    pub fn next_unsigned(&self) -> UnsignedCounter {
        self.u_counter + 1
    }

    pub fn prev_signed(&self) -> SignedCounter {
        self.s_counter - 1
    }

    pub fn default_signed_counter(&self) -> SignedCounter {
        0
    }

    pub fn default_unsigned_counter(&self) -> UnsignedCounter {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_signed() {
        let counter: Counter = Counter::new(10, 0);
        assert_eq!(counter.next_signed(), 11);
    }

    #[test]
    fn test_next_unsigned() {
        let counter: Counter = Counter::new(0, 10);
        assert_eq!(counter.next_unsigned(), 11);
    }

    #[test]
    fn test_prev_signed() {
        let counter: Counter = Counter::new(10, 0);
        assert_eq!(counter.prev_signed(), 9);
    }

    #[test]
    fn test_default_signed_counter() {
        let counter: Counter = Counter::new(0, 0);
        assert_eq!(counter.default_signed_counter(), 0);
    }

    #[test]
    fn test_default_unsigned_counter() {
        let counter: Counter = Counter::new(0, 0);
        assert_eq!(counter.default_unsigned_counter(), 0);
    }
}
