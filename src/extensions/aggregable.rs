use std::iter::{Product, Sum};

pub trait Aggregable<A>: IntoIterator<Item = A> + Sized {
  fn sum(self) -> A
  where
    A: Sum,
  {
    self.into_iter().sum()
  }

  fn product(self) -> A
  where
    A: Product,
  {
    self.into_iter().product()
  }
}
