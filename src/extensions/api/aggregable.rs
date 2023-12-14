use std::iter::{Product, Sum};

pub trait Aggregable<A> {
  fn sum(self) -> A
  where
    A: Sum;

  fn product(self) -> A
  where
    A: Product;
}
