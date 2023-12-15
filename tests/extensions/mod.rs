use cantrip::extensions::*;
use std::iter::{Product, Sum};

pub fn test_iterable<A, C>(data: C, mut predicate: impl FnMut(&A) -> bool) -> bool
where
  C: Iterable<A> + IntoIterator<Item = A> + Clone,
{
  data.all(|x| predicate(x)) == data.clone().into_iter().all(|x| predicate(&x))
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
