use std::collections::{BinaryHeap, BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;

use crate::assert_equal;

pub trait Equal {
  fn equal(&self, other: &Self) -> bool;
}

impl<Item: PartialEq> Equal for [Item] {
  fn equal(&self, other: &Self) -> bool {
    self == other
  }
}

impl<Item: PartialEq> Equal for LinkedList<Item> {
  fn equal(&self, other: &Self) -> bool {
    self == other
  }
}

impl<Item: PartialEq> Equal for Vec<Item> {
  fn equal(&self, other: &Self) -> bool {
    self == other
  }
}

impl<Item: PartialEq> Equal for VecDeque<Item> {
  fn equal(&self, other: &Self) -> bool {
    self == other
  }
}

impl<Item: Eq + Hash> Equal for HashSet<Item> {
  fn equal(&self, other: &Self) -> bool {
    self == other
  }
}

impl<Item: PartialEq> Equal for BTreeSet<Item> {
  fn equal(&self, other: &Self) -> bool {
    self == other
  }
}

impl<Item: PartialEq + Ord + Clone> Equal for BinaryHeap<Item> {
  fn equal(&self, other: &Self) -> bool {
    let mut self_values = self.clone().into_vec();
    let mut other_values = other.clone().into_vec();
    self_values.sort();
    other_values.sort();
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

//noinspection RsUnresolvedPath
pub(crate) fn assert_equal<C: FromIterator<i64> + Equal + Debug>(values: C, expected: &[i64]) {
  assert_equal!(values, from_slice(expected))
}

//noinspection RsUnresolvedPath
pub(crate) fn assert_map_equal<C: FromIterator<(i64, i64)> + Equal + Debug>(values: C, expected: &[(i64, i64)]) {
  assert_equal!(values, from_map_slice(expected))
}

pub(crate) fn from_slice<C: FromIterator<i64>>(values: &[i64]) -> C {
  C::from_iter(values.iter().copied())
}

pub(crate) fn from_map_slice<C: FromIterator<(i64, i64)>>(values: &[(i64, i64)]) -> C {
  C::from_iter(values.iter().copied())
}

#[macro_export]
macro_rules! assert_equal {
  ($left:expr, $right:expr $(,)?) => {
    match (&$left, &$right) {
      (left_value, right_value) => {
        if !(left_value.equal(right_value)) {
          panic!(
            r#"
assertion failed: {left_value:?} == {right_value:?}
"#
          )
        }
      }
    }
  };
}
