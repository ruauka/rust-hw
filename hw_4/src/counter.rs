pub type SignedCounter = isize;
pub type UnsignedCounter = usize;

#[derive(Debug)]
pub struct Counter {
    s_counter: SignedCounter,
    u_counter: UnsignedCounter,
}

impl Counter {
    // общий конструктор
    pub fn new(s_counter: isize, u_counter: usize) -> Self {
        Self {
            s_counter,
            u_counter,
        }
    }

    // конструктор под дефолтное значение signed
    pub fn default_signed_counter() -> Self {
        Self::new(1000, 0)
    }

    // конструктор под дефолтное значение unsigned
    pub fn default_unsigned_counter() -> Self {
        Self::new(0, 1000)
    }

    pub fn next_signed(&mut self) {
        self.s_counter += 1;
    }

    pub fn next_unsigned(&mut self) {
        self.u_counter += 1
    }

    pub fn prev_signed(&mut self) {
        self.s_counter -= 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_signed_counter() {
        let counter: Counter = Counter::default_signed_counter();
        assert_eq!(counter.s_counter, 1000);
        assert_eq!(counter.u_counter, 0);
    }

    #[test]
    fn test_default_unsigned_counter() {
        let counter: Counter = Counter::default_unsigned_counter();
        assert_eq!(counter.s_counter, 0);
        assert_eq!(counter.u_counter, 1000);
    }

    #[test]
    fn test_next_signed() {
        let mut counter: Counter = Counter::new(10, 0);
        counter.next_signed();

        assert_eq!(counter.s_counter, 11);
    }

    #[test]
    fn test_next_unsigned() {
        let mut counter: Counter = Counter::new(0, 10);
        counter.next_unsigned();

        assert_eq!(counter.u_counter, 11);
    }

    #[test]
    fn test_prev_signed() {
        let mut counter: Counter = Counter::new(10, 0);
        counter.prev_signed();

        assert_eq!(counter.s_counter, 9);
    }
}
