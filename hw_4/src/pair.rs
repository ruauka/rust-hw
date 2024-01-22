pub type Pair = (i32, i32);

#[derive(Debug)]
pub struct P {
    a_pair: Pair,
    b_pair: Pair,
}

impl P {
    pub fn new(a_pair: Pair, b_pair: Pair) -> Self {
        Self { a_pair, b_pair }
    }

    pub fn pair_vector_sum(&self) -> Pair {
        (self.a_pair.0 + self.b_pair.0, self.a_pair.1 + self.b_pair.1)
    }

    pub fn pair_scalar_sum(&self) -> i32 {
        self.a_pair.0 + self.a_pair.1 + self.b_pair.0 + self.b_pair.1
    }

    pub fn default_pair(&self) -> Pair {
        (0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pair_vector_sum() {
        let tup1: Pair = (1, 2);
        let tup2: Pair = (3, 4);

        let pair: P = P::new(tup1, tup2);

        assert_eq!(pair.pair_vector_sum(), (4, 6));
    }

    #[test]
    fn test_pair_scalar_sum() {
        let tup1: Pair = (1, 2);
        let tup2: Pair = (3, 4);

        let pair: P = P::new(tup1, tup2);

        assert_eq!(pair.pair_scalar_sum(), 10);
    }

    #[test]
    fn test_default_pair() {
        let empty_tuple: Pair = (0, 0);
        let pair: P = P::new(empty_tuple, empty_tuple);

        assert_eq!(pair.default_pair(), (0, 0));
    }
}
