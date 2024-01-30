pub mod array;
pub mod tuple;

#[derive(Clone, Copy)]
pub enum Item {
    First,
    Second,
    Third,
}

impl Item {
    pub fn index(&self) -> usize {
        match self {
            Item::First => 0,
            Item::Second => 1,
            Item::Third => 2,
        }
    }
}

pub trait Sequence {
    fn default_values() -> Self;
    fn get_item(&self, item: Item) -> f64;
    fn set_item(&mut self, item: Item, value: f64);
    fn is_default(&self) -> bool {
        self.default_check()
    }

    fn default_check(&self) -> bool;

    fn sum(&self) -> f64 {
        self.sum_count()
    }

    fn sum_count(&self) -> f64;
}

struct TestCase<T: Sequence> {
    obj: T,
    item: Item,
    expected: f64,
    test_name: String,
}

#[cfg(test)]
mod tests {
    use array::Array;
    use tuple::Tuple;
    use super::*;

    #[test]
    fn test_get_item() {
        let test_cases = vec![
            TestCase {
                obj: Tuple::new(100, 0.0, 0.0),
                item: Item::First,
                expected: 100_f64,
                test_name: "Tuple. get_item(). OK.".to_string(),
            },
            TestCase {
                obj: Array::new(100.0),
                item: Item::First,
                expected: 100_f64,
                test_name: "Array. get_item(). OK.".to_string(),
            },
        ];

        for test_case in test_cases.iter() {
            let actual: f64 = test_case.obj.get_item(test_case.item);
            assert_eq!(test_case.expected, actual)
        }

        // expected opaque type, found a different opaque type
    }
}
