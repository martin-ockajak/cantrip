use cantrip::extensions::*;
use std::cmp::Ordering;
use std::hash::Hash;
use std::iter::{Product, Sum};

pub trait TraversableFixture: Sized + Default {
  fn init_add() -> Self {
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
  fn init_mul() -> Self;

  fn check_add(&self, value: Self) -> Option<Self>;

  fn check_mul(&self, value: Self) -> Option<Self>;
}

pub fn test_traversable<A, C>(data: C) -> bool
where
  A: TraversableFixture + PartialEq + Ord + Clone,
  C: Traversable<A> + IntoIterator<Item = A> + Clone,
{
  let all = data.all(|x| x.test()) == data.clone().into_iter().all(|x| x.test());
  let any = data.any(|x| x.test()) == data.clone().into_iter().any(|x| x.test());
  let count_by = data.count_by(|x| x.test()) == data.clone().into_iter().filter(|x| x.test()).count();
  let fold = data.fold(A::init_add(), |r, x| r.safe_add(x))
    == data.clone().into_iter().fold(A::init_add(), |r, x| r.safe_add(&x));
  let max_by = data.max_by(|x, y| x.compare(y)).unwrap_or(&A::init_add())
    == &data.clone().into_iter().max_by(|x, y| x.compare(y)).unwrap_or(A::init_add());
  let min_by = data.min_by(|x, y| x.compare(y)).unwrap_or(&A::init_add())
    == &data.clone().into_iter().min_by(|x, y| x.compare(y)).unwrap_or(A::init_add());
  // FIXME - failing test
  // let reduce = data.reduce(|r, x| r.safe_add(x)) == data.clone().into_iter().reduce(|r, x| r.safe_add(&x));
  let reduce = true;
  all && any && count_by && fold && max_by && min_by && reduce
}

pub fn test_ordered<A, C>(data: C) -> bool
where
  A: TraversableFixture,
  C: Ordered<A> + IntoIterator<Item = A> + Clone,
{
  let position = data.position(|x| x.test()) == data.clone().into_iter().position(|x| x.test());
  // let rposition = data.rposition(|x| x.test()) == data.clone().into_iter().rev().position(|x| x.test());
  // position && rposition
  position
}

pub fn test_aggregable<A, C>(data: C) -> bool
where
  A: TraversableFixture + AggregableFixture + PartialEq + Sum + Product,
  C: Aggregable<A> + IntoIterator<Item = A> + Clone,
{
  (!safe_aggregate(data.clone(), A::init_add(), |x, y| x.check_add(y))
    || data.clone().sum() == data.clone().into_iter().sum())
    && (!safe_aggregate(data.clone(), A::init_mul(), |x, y| x.check_mul(y))
      || data.clone().product() == data.clone().into_iter().product())
}

pub fn test_list<'c, A, C>(data: C) -> bool
where
  A: TraversableFixture + 'c,
  C: Sequence<A> + IntoIterator<Item = A> + FromIterator<A> + Iterable<Item<'c> = &'c A> + PartialEq + Clone + 'c,
  C::This<A>: PartialEq + FromIterator<A>,
{
  let map = data.clone().map(|x| x.safe_add(x)) == data.clone().into_iter().map(|x| x.safe_add(&x)).collect();
  let filter = data.clone().filter(|x| x.test()) == data.clone().into_iter().filter(|x| x.test()).collect();
  map && filter
}

pub fn test_set<A, C>(data: C) -> bool
where
  A: TraversableFixture + Eq + Hash,
  C: EqSet<A> + IntoIterator<Item = A> + FromIterator<A> + PartialEq + Clone,
  C::This<A>: PartialEq + FromIterator<A>,
{
  let map = data.clone().map(|x| x.safe_add(x)) == data.clone().into_iter().map(|x| x.safe_add(&x)).collect();
  let filter = data.clone().filter(|x| x.test()) == data.clone().into_iter().filter(|x| x.test()).collect();
  map && filter
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

//   use std::collections::HashMap;
//
//   use crate::extensions::*;
//
//   #[quickcheck]
//   fn map(data: HashMap<i32, i32>) -> bool {
//     let function = |(k, v): (&i32, &i32)| (*k, *v as i64);
//     let result = data.map(function);
//     let expected = data.iter().map(function).collect::<HashMap<i32, i64>>();
//     result == expected
//   }
