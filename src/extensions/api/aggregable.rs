use std::iter::{Product, Sum};

pub trait Aggregable<A> {
  fn sum<S>(self) -> S
    where
      S: Sum<A>;

  fn product<S>(self) -> S
    where
      S: Product<A>;
}
