use std::hash::Hash;
use std::iter;
use std::iter::{Product, Sum};

use cantrip::extensions::*;

use crate::base::fixtures::{AggregableFixture, TraversableFixture};

// FIXME - add tests for all methods

pub fn test_iterable<A, C>(data: C) -> bool
where
  A: TraversableFixture + PartialEq + Ord + Clone,
  C: Iterable<A> + IntoIterator<Item = A> + FromIterator<A> + Clone,
{
  let all = data.all(|x| x.test()) == data.clone().into_iter().all(|x| x.test());
  let any = data.any(|x| x.test()) == data.clone().into_iter().any(|x| x.test());
  let count_by = data.count_by(|x| x.test()) == data.clone().into_iter().filter(|x| x.test()).count();
  let find = data.find(|x| x.test()) == data.clone().find(|x| x.test());
  let fold = data.fold(A::init_add(), |r, x| r.safe_add(x))
    == data.clone().into_iter().fold(A::init_add(), |r, x| r.safe_add(&x));
  let max_by = data.max_by(|x, y| x.compare(y)).unwrap_or(&A::init_add())
    == &data.clone().into_iter().max_by(|x, y| x.compare(y)).unwrap_or(A::init_add());
  let min_by = data.min_by(|x, y| x.compare(y)).unwrap_or(&A::init_add())
    == &data.clone().into_iter().min_by(|x, y| x.compare(y)).unwrap_or(A::init_add());
  let reduce = data.clone().reduce(|r, x| r.safe_add(&x)) == data.clone().into_iter().reduce(|r, x| r.safe_add(&x));
  all && any && count_by && find && fold && max_by && min_by
}

pub fn test_reverse_iterable<A, C, I>(data: C) -> bool
where
  A: TraversableFixture + PartialEq,
  C: ReverseIterable<A> + IntoIterator<Item = A, IntoIter = I> + Clone,
  I: Iterator<Item = A> + DoubleEndedIterator + ExactSizeIterator,
{
  let rfind = data.rfind(|x| x.test()) == data.clone().rfind(|x| x.test());
  let rfold = data.rfold(A::init_add(), |r, x| r.safe_add(x))
    == data.clone().into_iter().rfold(A::init_add(), |r, x| r.safe_add(&x));
  let size = data.clone().into_iter().len();
  let rposition =
    data.rposition(|x| x.test()) == data.clone().into_iter().rev().position(|x| x.test()).map(|x| size - x - 1);
  rfind && rfold && rposition
}

pub fn test_numeric<A, C>(data: C) -> bool
where
  A: TraversableFixture + AggregableFixture + PartialEq + Sum + Product,
  C: Collectible<A> + IntoIterator<Item = A> + Clone,
{
  (!safe_aggregate(data.clone(), A::init_add(), |x, y| x.check_add(y))
    || data.clone().sum() == data.clone().into_iter().sum())
    && (!safe_aggregate(data.clone(), A::init_mul(), |x, y| x.check_mul(y))
      || data.clone().product() == data.clone().into_iter().product())
}

pub fn test_collectible<A, C>(data: C) -> bool
where
  A: TraversableFixture + PartialEq,
  C: Collectible<A> + IntoIterator<Item = A> + Clone,
{
  true
}

pub fn test_sequence<'c, A, C, I>(data: C) -> bool
where
  A: TraversableFixture + 'c,
  C: Sequence<A> + IntoIterator<Item = A, IntoIter = I> + FromIterator<A> + PartialEq + Clone + 'c,
  C::This<A>: PartialEq + FromIterator<A>,
  C::This<(usize, A)>: PartialEq + FromIterator<(usize, A)>,
  I: Iterator<Item = A> + DoubleEndedIterator,
{
  let enumerate = data.clone().enumerate() == data.clone().into_iter().enumerate().collect();
  let filter = data.clone().filter(|x| x.test()) == data.clone().into_iter().filter(|x| x.test()).collect();
  let flat_map = data.clone().flat_map(|x| iter::once(x.safe_add(x)))
    == data.clone().into_iter().flat_map(|x| iter::once(x.safe_add(&x))).collect();
  let map = data.clone().map(|x| x.safe_add(x)) == data.clone().into_iter().map(|x| x.safe_add(&x)).collect();
  let position = data.position(|x| x.test()) == data.clone().into_iter().position(|x| x.test());
  let rev = data.clone().rev() == data.clone().into_iter().rev().collect();
  enumerate && filter && flat_map && map && position && rev
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
  C: Collectible<A> + IntoIterator<Item = A> + Clone,
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

//   use std::collections::HashMap;
//
//   use crate::base::*;
//
//   #[quickcheck]
//   fn map(data: HashMap<i32, i32>) -> bool {
//     let function = |(k, v): (&i32, &i32)| (*k, *v as i64);
//     let result = data.map(function);
//     let expected = data.iter().map(function).collect::<HashMap<i32, i64>>();
//     result == expected
//   }
