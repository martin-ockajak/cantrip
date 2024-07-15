use crate::assert_equal;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;

pub(crate) trait Equal {
  fn equal(&self, other: &Self) -> bool;
  //
  // fn compare(&self, other: &Self) -> Ordering;
}

impl<Item: PartialEq + Ord> Equal for [Item] {
  fn equal(&self, other: &Self) -> bool {
    self == other
  }
  //
  // fn compare(&self, other: &Self) -> Ordering {
  //   self.cmp(other)
  // }
}

impl<Item: PartialEq + Ord> Equal for LinkedList<Item> {
  fn equal(&self, other: &Self) -> bool {
    self == other
  }
  //
  // fn compare(&self, other: &Self) -> Ordering {
  //   self.cmp(other)
  // }
}

impl<Item: PartialEq + Ord> Equal for Vec<Item> {
  fn equal(&self, other: &Self) -> bool {
    self == other
  }
  //
  // fn compare(&self, other: &Self) -> Ordering {
  //   self.cmp(other)
  // }
}

impl<Item: PartialEq + Ord> Equal for VecDeque<Item> {
  fn equal(&self, other: &Self) -> bool {
    self == other
  }
  //
  // fn compare(&self, other: &Self) -> Ordering {
  //   self.cmp(other)
  // }
}

impl<Item: Eq + Hash + Ord + Clone> Equal for HashSet<Item> {
  fn equal(&self, other: &Self) -> bool {
    self == other
  }
  //
  // fn compare(&self, other: &Self) -> Ordering {
  //   let mut self_vec = Vec::from_iter(self.iter());
  //   let mut other_vec = Vec::from_iter(other.iter());
  //   self_vec.sort();
  //   other_vec.sort();
  //   self_vec.cmp(&other_vec)
  // }
}

impl<Item: PartialEq + Ord> Equal for BTreeSet<Item> {
  fn equal(&self, other: &Self) -> bool {
    self == other
  }
  //
  // fn compare(&self, other: &Self) -> Ordering {
  //   self.cmp(other)
  // }
}

impl<Item: Eq + Hash + Clone + Ord> Equal for BinaryHeap<Item> {
  fn equal(&self, other: &Self) -> bool {
    let self_values: HashSet<&Item> = HashSet::from_iter(self.iter());
    let other_values: HashSet<&Item> = HashSet::from_iter(other.iter());
    self_values == other_values
  }
  //
  // fn compare(&self, other: &Self) -> Ordering {
  //   let mut self_vec = Vec::from_iter(self.iter());
  //   let mut other_vec = Vec::from_iter(other.iter());
  //   self_vec.sort();
  //   other_vec.sort();
  //   self_vec.cmp(&other_vec)
  // }
}

impl<Key: Eq + Hash + Ord, Value: PartialEq + Ord> Equal for HashMap<Key, Value> {
  fn equal(&self, other: &Self) -> bool {
    self == other
  }
  //
  // fn compare(&self, other: &Self) -> Ordering {
  //   let mut self_vec = Vec::from_iter(self);
  //   let mut other_vec = Vec::from_iter(other);
  //   self_vec.sort();
  //   other_vec.sort();
  //   self_vec.cmp(&other_vec)
  // }
}

impl<Key: PartialEq + Ord, Value: PartialEq + Ord> Equal for BTreeMap<Key, Value> {
  fn equal(&self, other: &Self) -> bool {
    self == other
  }
  //
  // fn compare(&self, other: &Self) -> Ordering {
  //   let mut self_vec = Vec::from_iter(self);
  //   let mut other_vec = Vec::from_iter(other);
  //   self_vec.sort();
  //   other_vec.sort();
  //   self_vec.cmp(&other_vec)
  // }
}

//noinspection RsUnresolvedPath
pub(crate) fn assert_equal<T, C: FromIterator<T> + Equal + Debug>(values: C, expected: Vec<T>) {
  assert_equal!(values, C::from_iter(expected))
}

//noinspection RsUnresolvedPath
pub(crate) fn assert_set_equal<T: Eq + Hash + Ord + Clone + Debug, C: IntoIterator<Item = T> + Equal + Debug>(
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

//noinspection RsUnresolvedPath
pub(crate) fn assert_vec_set_equal<T: Ord + Clone + Debug, C: IntoIterator<Item = T> + Equal + Debug>(
  values_vec: Vec<C>, expected_vec: Vec<Vec<T>>,
) {
  let mut sorted_values_vec = Vec::from_iter(values_vec.into_iter().map(|item| {
    let mut item_vec = Vec::from_iter(item);
    item_vec.sort();
    item_vec
  }));
  sorted_values_vec.sort();
  let mut sorted_expected_vec = Vec::from_iter(expected_vec.into_iter().map(|item| {
    let mut item_vec = Vec::from_iter(item);
    item_vec.sort();
    item_vec
  }));
  sorted_expected_vec.sort();
  assert_equal!(sorted_values_vec, sorted_expected_vec)
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
