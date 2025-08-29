use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;

use cantrip::{Collection, CollectionTo, Iterable, Map, Sequence, SequenceTo};

use crate::assert_equal;

pub(crate) trait Equal {
  fn equal(&self, other: &Self) -> bool;
}

impl Equal for i64 {
  fn equal(&self, other: &Self) -> bool {
    self == other
  }
}

impl Equal for (i64, i64) {
  fn equal(&self, other: &Self) -> bool {
    self.0 == other.0 && self.1 == other.1
  }
}

impl Equal for (usize, i64) {
  fn equal(&self, other: &Self) -> bool {
    self.0 == other.0 && self.1 == other.1
  }
}

impl<Item: Equal> Equal for [Item] {
  fn equal(&self, other: &Self) -> bool {
    self.iter().zip(other.iter()).all(|(x, y)| x.equal(y))
  }
}

impl<Item: Equal> Equal for LinkedList<Item> {
  fn equal(&self, other: &Self) -> bool {
    self.iter().zip(other.iter()).all(|(x, y)| x.equal(y))
  }
}

impl<Item: Equal> Equal for Vec<Item> {
  fn equal(&self, other: &Self) -> bool {
    self.iter().zip(other.iter()).all(|(x, y)| x.equal(y))
  }
}

impl<Item: Equal> Equal for VecDeque<Item> {
  fn equal(&self, other: &Self) -> bool {
    self.iter().zip(other.iter()).all(|(x, y)| x.equal(y))
  }
}

impl<Item: Eq + Hash + Clone> Equal for HashSet<Item> {
  fn equal(&self, other: &Self) -> bool {
    self == other
  }
}

impl<Item: PartialEq> Equal for BTreeSet<Item> {
  fn equal(&self, other: &Self) -> bool {
    self == other
  }
}

impl<Item: Eq + Hash + Clone> Equal for BinaryHeap<Item> {
  fn equal(&self, other: &Self) -> bool {
    let self_values: HashSet<&Item> = self.iter().collect();
    let other_values: HashSet<&Item> = other.iter().collect();
    self_values == other_values
  }
}

impl<Key: Eq + Hash, Value: PartialEq> Equal for HashMap<Key, Value> {
  fn equal(&self, other: &Self) -> bool {
    self == other
  }
}

impl<Key: PartialEq, Value: PartialEq> Equal for BTreeMap<Key, Value> {
  fn equal(&self, other: &Self) -> bool {
    self == other
  }
}

pub(crate) trait TestCollection<T>: FromIterator<T> + Default + Extend<T> + Clone + Equal + Debug {}

pub(crate) trait TestCollectible<'a, T: 'a>:
  CollectionTo<T> + TestCollection<T> + IntoIterator<Item = T> + Iterable<Item<'a> = &'a T>
where
  Self: 'a,
{
}

pub(crate) trait TestSequence<'a, T: 'a, I>:
  Collection<T>
  + CollectionTo<T>
  + Sequence<T>
  + SequenceTo<T>
  + TestCollection<T>
  + IntoIterator<Item = i64, IntoIter = I>
  + Iterable<Item<'a> = &'a T>
where
  I: DoubleEndedIterator<Item = i64> + ExactSizeIterator<Item = i64>,
  Self: 'a,
{
}

pub(crate) trait TestMap<'a, K: 'a, V: 'a>:
  Map<K, V>
  + FromIterator<(K, V)>
  + Default
  + Extend<(K, V)>
  + Clone
  + Equal
  + Debug
  + IntoIterator<Item = (K, V)>
  + Iterable<Item<'a> = (&'a K, &'a V)>
where
  Self: 'a,
{
}

impl<T, C> TestCollection<T> for C where C: FromIterator<T> + Default + Extend<T> + Clone + Equal + Debug {}

impl<'a, T: 'a, C> TestCollectible<'a, T> for C where
  C: TestCollection<T> + CollectionTo<T> + IntoIterator<Item = T> + Iterable<Item<'a> = &'a T> + 'a
{
}

impl<'a, T: 'a, C, I> TestSequence<'a, T, I> for C
where
  I: DoubleEndedIterator<Item = i64> + ExactSizeIterator<Item = i64>,
  C: Collection<T>
    + CollectionTo<T>
    + Sequence<T>
    + SequenceTo<T>
    + TestCollection<T>
    + IntoIterator<Item = i64, IntoIter = I>
    + Iterable<Item<'a> = &'a T>
    + 'a,
{
}

impl<'a, K: 'a, V: 'a, C> TestMap<'a, K, V> for C where
  C: Map<K, V>
    + FromIterator<(K, V)>
    + Default
    + Extend<(K, V)>
    + Clone
    + Equal
    + Debug
    + IntoIterator<Item = (K, V)>
    + Iterable<Item<'a> = (&'a K, &'a V)>
    + 'a
{
}

// noinspection RsUnresolvedPath
pub(crate) fn assert_seq_equal<T, C: FromIterator<T> + Equal + Debug>(values: &C, expected: Vec<T>) {
  assert_equal!(values, &C::from_iter(expected));
}

// noinspection RsUnresolvedPath
pub(crate) fn assert_set_equal<T: Equal + Eq + Hash + Clone + Debug, C: IntoIterator<Item = T> + Equal + Debug>(
  values: C, expected: Vec<T>,
) {
  let values_set = HashSet::from_iter(values);
  let expected_set = HashSet::from_iter(expected);
  assert_equal!(values_set, expected_set);
}

// noinspection RsUnresolvedPath
pub(crate) fn assert_map_equal<K: Debug, V: Debug, C: FromIterator<(K, V)> + Equal + Debug>(
  values: &C, expected: HashMap<K, V>,
) {
  assert_equal!(values, &C::from_iter(expected));
}

// noinspection RsUnresolvedPath
pub(crate) fn assert_vec_seq_equal<T: Ord + Debug, C: IntoIterator<Item = T> + Debug>(
  values: Vec<C>, expected: &Vec<Vec<T>>,
) {
  let values_vec = values.into_iter().map(|x| Vec::from_iter(x)).collect::<Vec<_>>();
  assert_eq!(&values_vec, expected);
}

// noinspection RsUnresolvedPath
pub(crate) fn assert_vec_seq_equivalent<T: Equal + Ord + Clone + Debug, C: IntoIterator<Item = T> + Equal + Debug>(
  values: Vec<C>, expected: Vec<Vec<T>>,
) {
  let mut values_sorted = values
    .into_iter()
    .map(|x| {
      let mut items = Vec::from_iter(x);
      items.sort();
      items
    })
    .collect::<Vec<_>>();
  values_sorted.sort();
  let mut expected_sorted = expected
    .into_iter()
    .map(|x| {
      let mut items = Vec::from_iter(x);
      items.sort();
      items
    })
    .collect::<Vec<_>>();
  expected_sorted.sort();
  assert_seq_equal(&values_sorted, expected_sorted);
}

// noinspection RsUnresolvedPath
pub(crate) fn assert_map_vec_equivalent<
  K: Eq + Hash + Debug,
  V: Ord + Debug,
  C: IntoIterator<Item = V> + Equal + Debug,
>(
  values: HashMap<K, C>, expected: HashMap<K, Vec<V>>,
) {
  let values_sorted = values
    .into_iter()
    .map(|(k, v)| {
      let mut items = Vec::from_iter(v);
      items.sort();
      (k, items)
    })
    .collect::<HashMap<_, _>>();
  let expected_sorted = expected
    .into_iter()
    .map(|(k, v)| {
      let mut items = Vec::from_iter(v);
      items.sort();
      (k, items)
    })
    .collect::<HashMap<_, _>>();
  assert_map_equal(&values_sorted, expected_sorted);
}

#[macro_export]
macro_rules! assert_equal {
  ($left:expr, $right:expr $(,)?) => {
    match (&$left, &$right) {
      (left_value, right_value) => {
        if !(left_value.equal(right_value)) {
          panic!("assertion failed: {left_value:?} == {right_value:?}")
        }
      }
    }
  };
}
