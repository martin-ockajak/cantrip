use crate::assert_equal;
use cantrip::{Collectible, Map, Ordered, Sequence, Traversable};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;

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
    let self_values: HashSet<&Item> = HashSet::from_iter(self.iter());
    let other_values: HashSet<&Item> = HashSet::from_iter(other.iter());
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

pub(crate) trait TestSet<T>: Traversable<T> + Collectible<T> + TestCollection<T> {}

pub(crate) trait TestSequence<T>:
  Traversable<T> + Collectible<T> + Ordered<T> + Sequence<T> + TestCollection<T>
{
}

pub(crate) trait TestMap<K, V>: Map<K, V> + TestCollection<(K, V)> + IntoIterator<Item = (K, V)> {}

impl<T: Clone + Equal + Debug> TestCollection<T> for Vec<T> {}

impl<T: Clone + Equal + Debug> TestCollection<T> for VecDeque<T> {}

impl<T: Clone + Equal + Debug> TestCollection<T> for LinkedList<T> {}

impl<T: Clone + Equal + Debug + Eq + Hash> TestCollection<T> for HashSet<T> {}

impl<T: Clone + Equal + Debug + Ord> TestCollection<T> for BTreeSet<T> {}

impl<T: Clone + Equal + Debug + Ord + Eq + Hash> TestCollection<T> for BinaryHeap<T> {}

impl<K: Clone + Equal + Debug + Eq + Hash, V: Clone + Equal + PartialEq + Debug> TestCollection<(K, V)> for HashMap<K, V> {}

impl<K: Clone + Equal + Debug + Ord, V: Clone + Equal + PartialEq + Debug> TestCollection<(K, V)> for BTreeMap<K, V> {}

impl<T: Clone + Equal + Debug> TestSequence<T> for Vec<T> {}

impl<T: Clone + Equal + Debug> TestSequence<T> for VecDeque<T> {}

impl<T: Clone + Equal + Debug> TestSequence<T> for LinkedList<T> {}

impl<T: Clone + Equal + Debug + Eq + Hash> TestSet<T> for HashSet<T> {}

impl<T: Clone + Equal + Debug + Ord> TestSet<T> for BTreeSet<T> {}

impl<T: Clone + Equal + Debug + Ord + Eq + Hash> TestSet<T> for BinaryHeap<T> {}

impl<K: Clone + Equal + Debug + Eq + Hash, V: Clone + Equal + PartialEq + Debug> TestMap<K, V> for HashMap<K, V> {}

impl<K: Clone + Equal + Debug + Ord, V: Clone + Equal + PartialEq + Debug> TestMap<K, V> for BTreeMap<K, V> {}

//noinspection RsUnresolvedPath
pub(crate) fn assert_seq_equal<T, C: FromIterator<T> + Equal + Debug>(values: C, expected: Vec<T>) {
  assert_equal!(values, C::from_iter(expected))
}

//noinspection RsUnresolvedPath
pub(crate) fn assert_set_equal<T: Equal + Eq + Hash + Clone + Debug, C: IntoIterator<Item = T> + Equal + Debug>(
  values: C, expected: Vec<T>,
) {
  let values_set = HashSet::from_iter(values);
  let expected_set = HashSet::from_iter(expected);
  assert_equal!(values_set, expected_set)
}

//noinspection RsUnresolvedPath
pub(crate) fn assert_map_equal<K: Debug, V: Debug, C: FromIterator<(K, V)> + Equal + Debug>(
  values: C, expected: HashMap<K, V>,
) {
  assert_equal!(values, C::from_iter(expected))
}

//noinspection RsUnresolvedPath
pub(crate) fn assert_vec_seq_equal<T: Ord + Debug, C: IntoIterator<Item = T> + Debug>(
  values: Vec<C>, expected: Vec<Vec<T>>,
) {
  let values_vec = Vec::from_iter(values.into_iter().map(|x| Vec::from_iter(x)));
  assert_eq!(values_vec, expected)
}

//noinspection RsUnresolvedPath
pub(crate) fn assert_vec_seq_equivalent<T: Equal + Ord + Clone + Debug, C: IntoIterator<Item = T> + Equal + Debug>(
  values: Vec<C>, expected: Vec<Vec<T>>,
) {
  let mut values_sorted = Vec::from_iter(values.into_iter().map(|x| {
    let mut items = Vec::from_iter(x);
    items.sort();
    items
  }));
  values_sorted.sort();
  let mut expected_sorted = Vec::from_iter(expected.into_iter().map(|x| {
    let mut items = Vec::from_iter(x);
    items.sort();
    items
  }));
  expected_sorted.sort();
  assert_seq_equal(values_sorted, expected_sorted)
}

//noinspection RsUnresolvedPath
pub(crate) fn assert_map_vec_equivalent<
  K: Eq + Hash + Debug,
  V: Ord + Debug,
  C: IntoIterator<Item = V> + Equal + Debug,
>(
  values: HashMap<K, C>, expected: HashMap<K, Vec<V>>,
) {
  let values_sorted = HashMap::from_iter(values.into_iter().map(|(k, v)| {
    let mut items = Vec::from_iter(v);
    items.sort();
    (k, items)
  }));
  let expected_sorted = HashMap::from_iter(expected.into_iter().map(|(k, v)| {
    let mut items = Vec::from_iter(v);
    items.sort();
    (k, items)
  }));
  assert_map_equal(values_sorted, expected_sorted);
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
