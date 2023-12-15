use cantrip::extensions::*;
use std::cmp::Ordering;
use std::iter::{Product, Sum};

pub fn test_iterable<A, C>(
  data: C, init: A, mut predicate: impl FnMut(&A) -> bool, mut add: impl FnMut(A, &A) -> A,
  mut compare: impl FnMut(&A, &A) -> Ordering,
) -> bool
where
  A: PartialEq + Clone,
  C: Iterable<A> + IntoIterator<Item = A> + Clone,
{
  data.all(|x| predicate(x)) == data.clone().into_iter().all(|x| predicate(&x))
    && data.any(|x| predicate(x)) == data.clone().into_iter().any(|x| predicate(&x))
    && data.count_by(|x| predicate(x)) == data.clone().into_iter().filter(predicate).count()
    && data.fold(init.clone(), |r, x| add(r, x)) == data.clone().into_iter().fold(init.clone(), |r, x| add(r, &x))
    && data.max_by(|x, y| compare(x, y)).unwrap_or(&init.clone())
      == &data.clone().into_iter().max_by(|x, y| compare(x, y)).unwrap_or(init.clone())
    && data.min_by(|x, y| compare(x, y)).unwrap_or(&init.clone())
    == &data.clone().into_iter().min_by(|x, y| compare(x, y)).unwrap_or(init.clone())
}

pub fn test_ordered<A, C>(data: C, mut predicate: impl FnMut(&A) -> bool) -> bool
where
  C: Ordered<A> + IntoIterator<Item = A> + Clone,
{
  data.position(|x| predicate(x)) == data.clone().into_iter().position(|x| predicate(&x))
}

pub fn test_aggregable<A, C>(
  data: C, init_add: A, checked_add: impl FnMut(A, A) -> Option<A>, init_mul: A,
  checked_mul: impl FnMut(A, A) -> Option<A>,
) -> bool
where
  A: PartialEq + Sum + Product,
  C: Aggregable<A> + IntoIterator<Item = A> + Clone,
{
  (!safe_aggregate(data.clone(), init_add, checked_add) || data.clone().sum() == data.clone().into_iter().sum())
    && (!safe_aggregate(data.clone(), init_mul, checked_mul)
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
