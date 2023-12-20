use std::iter::{Product, Sum};

pub trait Summable<Item>: IntoIterator<Item = Item> + Sized {
  fn sum(self) -> Item
  where
    Item: Sum,
  {
    self.into_iter().sum()
  }
}
