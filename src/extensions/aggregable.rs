use std::iter::{Product, Sum};

pub trait Aggregable<Item>: IntoIterator<Item = Item> + Sized {
  fn sum(self) -> Item
  where
    Item: Sum,
  {
    self.into_iter().sum()
  }

  fn product(self) -> Item
  where
    Item: Product,
  {
    self.into_iter().product()
  }
}
