use cantrip::extensions::*;
use std::cmp::Ordering;
use std::iter::{Product, Sum};

pub trait IterableFixture: Sized + Default {
  fn init() -> Self {
    Self::default()
  }

  fn test(&self) -> bool;

  fn safe_add(&self, value: &Self) -> Self;

  fn compare(&self, value: &Self) -> Ordering
  where
    Self: Ord,
  {
    self.cmp(value)
  }
}

pub trait AggregableFixture: Sized + Default {
  fn init_add() -> Self {
    Self::default()
  }

  fn init_mul() -> Self;

  fn safe_add(&self, value: Self) -> Option<Self>;

  fn safe_mul(&self, value: Self) -> Option<Self>;
}

pub fn test_iterable<A, C>(data: C) -> bool
where
  A: IterableFixture + PartialEq + Ord + Clone,
  C: Iterable<A> + IntoIterator<Item = A> + Clone,
{
  data.all(|x| x.test()) == data.clone().into_iter().all(|x| x.test())
    && data.any(|x| x.test()) == data.clone().into_iter().any(|x| x.test())
    && data.count_by(|x| x.test()) == data.clone().into_iter().filter(|x| x.test()).count()
    && data.fold(A::init(), |r, x| r.safe_add(x)) == data.clone().into_iter().fold(A::init(), |r, x| r.safe_add(&x))
    && data.max_by(|x, y| x.compare(y)).unwrap_or(&A::init())
      == &data.clone().into_iter().max_by(|x, y| x.compare(y)).unwrap_or(A::init())
    && data.min_by(|x, y| x.compare(y)).unwrap_or(&A::init())
      == &data.clone().into_iter().min_by(|x, y| x.compare(y)).unwrap_or(A::init())
}

pub fn test_ordered<A, C>(data: C) -> bool
where
  A: IterableFixture,
  C: Ordered<A> + IntoIterator<Item = A> + Clone,
{
  data.position(|x| x.test()) == data.clone().into_iter().position(|x| x.test())
}

pub fn test_aggregable<A, C>(data: C) -> bool
where
  A: AggregableFixture + PartialEq + Sum + Product,
  C: Aggregable<A> + IntoIterator<Item = A> + Clone,
{
  (!safe_aggregate(data.clone(), A::init_add(), |x, y| x.safe_add(y))
    || data.clone().sum() == data.clone().into_iter().sum())
    && (!safe_aggregate(data.clone(), A::init_mul(), |x, y| x.safe_mul(y))
      || data.clone().product() == data.clone().into_iter().product())
}

fn safe_aggregate<A, C>(data: C, init: A, mut aggregate: impl FnMut(A, A) -> Option<A>) -> bool
where
  A: PartialEq + Sum + Product,
  C: Aggregable<A> + IntoIterator<Item = A> + Clone,
{
  let mut result = init;
  let mut safe = true;
  for value in data.into_iter() {
    match aggregate(result, value) {
      Some(number) => {
        result = number;
      }
      None => {
        safe = false;
        break;
      }
    }
  }
  safe
}

// pub fn test_list<A, C>(data: C, mut predicate: impl FnMut(&A) -> bool) -> bool
//   where
//     C: List<A> + IntoIterator<Item = A> + Clone + PartialEq,
// {
//   data.clone().rev() == data.clone().into_iter().rev().collect()
// }
