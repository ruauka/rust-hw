use crate::{Item, Sequence};

#[derive(Debug, PartialEq)]
pub struct Tuple(u32, f32, f64);

impl Sequence for Tuple {
    fn default_values() -> Self {
        Self(0, 0.0, 0.0)
    }

    fn get_item(&self, item: Item) -> f64 {
        match item {
            Item::First => self.0 as _,
            Item::Second => self.1 as _,
            Item::Third => self.2,
        }
    }

    fn set_item(&mut self, item: Item, value: f64) {
        match item {
            Item::First => self.0 = value as _,
            Item::Second => self.1 = value as _,
            Item::Third => self.2 = value,
        };
    }

    fn default_check(&self) -> bool {
        self.0 == 0 && self.1 == 0.0 && self.2 == 0.0
    }

    fn sum_count(&self) -> f64 {
        self.0 as f64 + self.1 as f64 + self.2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_values() {
        let expected: Tuple = Tuple {
            0: 0,
            1: 0.0,
            2: 0.0,
        };

        let actual: Tuple = Tuple::default_values();

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_get_item() {
        let mut tuple: Tuple = Tuple::default_values();
        tuple.0 = 100;

        let actual: f64 = tuple.get_item(Item::First);

        assert_eq!(100_f64, actual)
    }

    #[test]
    fn test_set_item() {
        let mut tuple: Tuple = Tuple::default_values();
        tuple.set_item(Item::First, 100_f64);

        let actual: u32 = tuple.0;

        assert_eq!(100_u32, actual)
    }

    #[test]
    fn test_default_check() {
        let tuple: Tuple = Tuple::default_values();
        let actual: bool = tuple.is_default();

        assert_eq!(true, actual)
    }

    #[test]
    fn test_sum_count() {
        let mut tuple: Tuple = Tuple::default_values();
        tuple.0 = 100;

        let actual: f64 = tuple.sum();

        assert_eq!(100_f64, actual)
    }
}
