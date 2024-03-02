use crate::{Item, Sequence};

#[derive(Debug, PartialEq)]
pub struct Array([f64; 3]);

impl Array {
    pub fn new(value: f64) -> impl Sequence {
        Array([value; 3])
    }
}

impl Sequence for Array {
    fn default_values() -> Self {
        Self([0.0; 3])
    }

    fn get_item(&self, item: Item) -> f64 {
        self.0[item.index()]
    }

    fn set_item(&mut self, item: Item, value: f64) {
        self.0[item.index()] = value
    }

    fn default_check(&self) -> bool {
        for value in &self.0 {
            if *value != 0.0 {
                return false;
            }
        }
        true
    }

    fn sum_count(&self) -> f64 {
        let mut sum = 0.0;
        for value in &self.0 {
            sum += *value;
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_values() {
        let expected: Array = Array { 0: [0_f64; 3] };

        let actual: Array = Array::default_values();

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_get_item() {
        let mut arr: Array = Array::default_values();
        arr.0 = [100_f64; 3];

        let actual: f64 = arr.get_item(Item::First);

        assert_eq!(100_f64, actual)
    }

    #[test]
    fn test_set_item() {
        let mut arr: Array = Array::default_values();
        arr.set_item(Item::First, 100_f64);

        let actual: f64 = arr.0[0];

        assert_eq!(100_f64, actual)
    }

    #[test]
    fn test_default_check() {
        let arr: Array = Array::default_values();
        let actual: bool = arr.is_default();
        assert_eq!(true, actual);

        let mut arr: Array = Array::default_values();
        arr.0 = [100_f64; 3];
        let actual: bool = arr.is_default();
        assert_eq!(false, actual)
    }

    #[test]
    fn test_sum_count() {
        let mut arr: Array = Array::default_values();
        arr.0 = [100_f64; 3];

        let actual: f64 = arr.sum();

        assert_eq!(300_f64, actual)
    }
}
