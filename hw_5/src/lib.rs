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
