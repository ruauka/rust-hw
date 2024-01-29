pub type Pair = (i32, i32);

#[derive(Debug)]
pub struct PairStruct {
    a_pair: Pair,
    b_pair: Pair,
}

impl PairStruct {
    // общий конструктор
    pub fn new(a_pair: Pair, b_pair: Pair) -> Self {
        Self { a_pair, b_pair }
    }

    // конструктор под дефолтное значение
    pub fn default_pair() -> Self {
        Self::new((100, 100), (200, 200))
    }

    pub fn pair_vector_sum(&self) -> Pair {
        (self.a_pair.0 + self.b_pair.0, self.a_pair.1 + self.b_pair.1)
    }

    pub fn pair_scalar_sum(&self) -> i32 {
        self.a_pair.0 + self.a_pair.1 + self.b_pair.0 + self.b_pair.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_pair() {
        let pair: PairStruct = PairStruct::default_pair();

        assert_eq!(pair.a_pair, (100, 100));
        assert_eq!(pair.b_pair, (200, 200));
    }

    #[test]
    fn test_pair_vector_sum() {
        let tup1: Pair = (1, 2);
        let tup2: Pair = (3, 4);

        let pair: PairStruct = PairStruct::new(tup1, tup2);

        assert_eq!(pair.pair_vector_sum(), (4, 6));
    }

    #[test]
    fn test_pair_scalar_sum() {
        let tup1: Pair = (1, 2);
        let tup2: Pair = (3, 4);

        let pair: PairStruct = PairStruct::new(tup1, tup2);

        assert_eq!(pair.pair_scalar_sum(), 10);
    }
}
