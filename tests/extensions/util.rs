use crate::assert_equal;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;

pub(crate) trait Equal {
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

impl<Item: Eq + Hash + Clone> Equal for BinaryHeap<Item> {
  fn equal(&self, other: &Self) -> bool {
    let self_values: HashSet<Item> = HashSet::from_iter(self.clone());
    let other_values: HashSet<Item> = HashSet::from_iter(other.clone());
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
pub(crate) fn assert_equal<T, C: FromIterator<T> + Equal + Debug>(values: C, expected: Vec<T>) {
  assert_equal!(values, C::from_iter(expected))
}

//noinspection RsUnresolvedPath
pub(crate) fn assert_set_equal<T: Eq + Hash + Debug, C: IntoIterator<Item = T> + Equal + Debug>(
  values: C, expected: Vec<T>,
) {
  let values_set = HashSet::from_iter(values);
  let expected_set = HashSet::from_iter(expected);
  assert_equal!(values_set, expected_set)
}

//noinspection RsUnresolvedPath
pub(crate) fn assert_map_equal<K, V, C: FromIterator<(K, V)> + Equal + Debug>(values: C, expected: Vec<(K, V)>) {
  assert_equal!(values, C::from_iter(expected))
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
