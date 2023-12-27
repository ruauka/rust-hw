pub type Pair = (i32, i32);

pub fn pair_vector_sum(a: Pair, b: Pair) -> Pair {
    (a.0 + b.0, a.1 + b.1)
}

pub fn pair_scalar_sum(a: Pair, b: Pair) -> i32 {
    a.0 + a.1 + b.0 + b.1
}

pub fn default_pair() -> Pair {
    (0, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pair_vector_sum() {
        let tup1: (i32, i32) = (1, 2);
        let tup2: (i32, i32) = (3, 4);

        assert_eq!(pair_vector_sum(tup1, tup2), (4, 6));
    }

    #[test]
    fn test_pair_scalar_sum() {
        let tup1: (i32, i32) = (1, 2);
        let tup2: (i32, i32) = (3, 4);

        assert_eq!(pair_scalar_sum(tup1, tup2), 10);
    }

    #[test]
    fn test_default_pair() {
        assert_eq!(default_pair(), (0, 0));
    }
}
