use std::iter::{Product, Sum};

pub trait Productable<Item>: IntoIterator<Item = Item> + Sized {
  fn product(self) -> Item
  where
    Item: Product,
  {
    self.into_iter().product()
  }
}
