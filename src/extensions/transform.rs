use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};
use std::hash::Hash;

use crate::Iterable;

/// Non-consuming transform operations.
///
/// Methods have the following properties:
///
/// - Requires collection elements to implement [`Clone`]
/// - Does not consume the collection or its elements
/// - Creates a new collection
///
pub trait Transform<Item> {
  /// Transforms this collection into specified collection type.
  ///
  /// `collect()` can take any collection, and turn it into a relevant
  /// collection. This can be used in a variety of contexts.
  ///
  /// `collect()` can also create instances of types that are not typical
  /// collections. For example, a [`String`] can be built from [`char`]s,
  /// and a collection of [`Result<T, E>`][`Result`] items can be collected
  /// into `Result<Collection<T>, E>`. See the examples below for more.
  ///
  /// Because `collect()` is so general, it can cause problems with type
  /// inference. As such, `collect()` is one of the few times you'll see
  /// the syntax affectionately known as the 'turbofish': `::<>`. This
  /// helps the inference algorithm understand specifically which collection
  /// you're trying to collect into.
  ///
  /// This is a non-consuming variant of [`collect_to()`].
  ///
  /// [`collect_to()`]: crate::CollectionTo::collect_to
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::{LinkedList, VecDeque};
  ///
  /// let a = VecDeque::from([1, 2, 3]);
  ///
  /// let collected: LinkedList<i32> = a.collect_to();
  ///
  /// assert_eq!(collected, LinkedList::from([1, 2, 3]));
  /// ```
  ///
  /// Note that we needed the `::LinkedList<i32>` on the left-hand side. This is because
  /// we could collect into, for example, a [`VecDeque<T>`] instead:
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::{LinkedList, VecDeque};
  ///
  /// let a = LinkedList::from([1, 2, 3]);
  ///
  /// let collected: VecDeque<i32> = a.collect_to();
  ///
  /// assert_eq!(collected, VecDeque::from([1, 2, 3]));
  /// ```
  ///
  /// Using the 'turbofish' instead of annotating `collected`:
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::{LinkedList, VecDeque};
  ///
  /// let a = LinkedList::from([1, 2, 3]);
  ///
  /// assert_eq!(a.collect_to::<VecDeque<i32>>(), VecDeque::from([1, 2, 3]));
  /// ```
  ///
  /// Because `collect()` only cares about what you're collecting into, you can
  /// still use a partial type hint, `_`, with the turbofish:
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::{LinkedList, VecDeque};
  ///
  /// let a = LinkedList::from([1, 2, 3]);
  ///
  /// assert_eq!(a.collect_to::<VecDeque<_>>(), VecDeque::from([1, 2, 3]));
  /// ```
  ///
  /// Using `collect()` to make a [`String`]:
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::LinkedList;
  ///
  /// let a = LinkedList::from(['h', 'e', 'l', 'l', 'o']);
  ///
  /// let hello: String = a.collect_to();
  ///
  /// assert_eq!("hello", hello);
  /// ```
  ///
  /// If you have a list of [`Result<T, E>`][`Result`]s, you can use `collect()` to
  /// see if any of them failed:
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::LinkedList;
  ///
  /// let a = LinkedList::from([Ok(1), Err("nope"), Ok(3), Err("bad")]);
  ///
  /// let result: Result<Vec<_>, &str> = a.collect_to();
  ///
  /// // gives us the first error
  /// assert_eq!(Err("nope"), result);
  ///
  /// let b = LinkedList::from([Ok(1), Ok(3)]);
  ///
  /// let result: Result<Vec<_>, &str> = b.collect_to();
  ///
  /// // gives us the list of answers
  /// assert_eq!(Ok(vec![1, 3]), result);
  /// ```
  ///
  /// [`VecDeque<T>`]: ../../std/collections/struct.VecDeque.html
  /// [`iter`]: Iterator::next
  /// [`String`]: ../../std/string/struct.String.html
  /// [`char`]: type@char
  fn collect<B>(&self) -> B
  where
    Item: Clone,
    B: FromIterator<Item>;

  /// Creates a new ordered set from the elements of this collection.
  ///
  /// This is a non-consuming equivalent of [`Iterator::collect`].
  ///
  /// This is a non-consuming variant of [`into_bset()`].
  ///
  /// [`into_bset()`]: crate::TransformTo::into_bset
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::{BTreeSet, LinkedList};
  ///
  /// let a = LinkedList::from([1, 2, 3]);
  ///
  /// assert_eq!(a.to_bset(), BTreeSet::from([1, 2, 3]));
  /// ```
  fn to_bset(&self) -> BTreeSet<Item>
  where
    Item: Ord + Clone;

  /// Creates a new ordered map from the elements of this collection.
  ///
  /// This is a non-consuming equivalent of [`Iterator::collect`].
  ///
  /// This is a non-consuming variant of [`into_bmap()`].
  ///
  /// [`into_bmap()`]: crate::TransformTo::into_bmap
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::{BTreeMap, LinkedList};
  ///
  /// let a = LinkedList::from([(1, 1), (2, 2), (3, 3)]);
  ///
  /// assert_eq!(a.to_bmap(), BTreeMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3)
  /// ]));
  /// ```
  #[inline]
  fn to_bmap<'a, K, V>(&'a self) -> BTreeMap<K, V>
  where
    K: Ord + Clone + 'a,
    V: Clone + 'a,
    Self: Iterable<Item<'a> = &'a (K, V)> + 'a,
  {
    self.iterator().cloned().collect()
  }

  /// Creates a new double-ended queue from the elements of this collection.
  ///
  /// This is a non-consuming equivalent of [`Iterator::collect`].
  ///
  /// This is a non-consuming variant of [`into_deque()`].
  ///
  /// [`into_deque()`]: crate::TransformTo::into_deque
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::{LinkedList, VecDeque};
  ///
  /// let a = LinkedList::from([1, 2, 3]);
  ///
  /// assert_eq!(a.to_deque(), VecDeque::from([1, 2, 3]));
  /// ```
  fn to_deque(&self) -> VecDeque<Item>
  where
    Item: Clone;

  /// Creates a new priority queue from the elements of this collection.
  ///
  /// This is a non-consuming equivalent of [`Iterator::collect`].
  ///
  /// This is a non-consuming variant of [`into_heap()`].
  ///
  /// [`into_heap()`]: crate::TransformTo::into_heap
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::{BinaryHeap, HashSet, LinkedList};
  ///
  /// let a = LinkedList::from([1, 2, 3]);
  ///
  /// assert_eq!(
  ///   a.to_heap().into_iter().collect::<HashSet<_>>(),
  ///   BinaryHeap::from([1, 2, 3]).into_iter().collect()
  /// );
  /// ```
  fn to_heap(&self) -> BinaryHeap<Item>
  where
    Item: Ord + Clone;

  /// Creates a new doubly-linked list from the elements of this collection.
  ///
  /// This is a non-consuming equivalent of [`Iterator::collect`].
  ///
  /// This is a non-consuming variant of [`into_list()`].
  ///
  /// [`into_list()`]: crate::TransformTo::into_list
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::{LinkedList, VecDeque};
  ///
  /// let a = VecDeque::from([1, 2, 3]);
  ///
  /// assert_eq!(a.to_list(), LinkedList::from([1, 2, 3]));
  /// ```
  fn to_list(&self) -> LinkedList<Item>
  where
    Item: Clone;

  /// Creates a new hash map from the elements of this collection.
  ///
  /// This is a non-consuming equivalent of [`Iterator::collect`].
  ///
  /// This is a non-consuming variant of [`into_map()`].
  ///
  /// [`into_map()`]: crate::TransformTo::into_map
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::{HashMap, LinkedList};
  ///
  /// let a = LinkedList::from([(1, 1), (2, 2), (3, 3)]);
  ///
  /// assert_eq!(a.to_map(), HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3)
  /// ]));
  /// ```
  #[inline]
  fn to_map<'a, K, V>(&'a self) -> HashMap<K, V>
  where
    K: Eq + Hash + Clone + 'a,
    V: Clone + 'a,
    Self: Iterable<Item<'a> = &'a (K, V)> + 'a,
  {
    self.iterator().cloned().collect()
  }

  /// Creates a new hash set from the elements of this collection.
  ///
  /// This is a non-consuming equivalent of [`Iterator::collect`].
  ///
  /// This is a non-consuming variant of [`into_set()`].
  ///
  /// [`into_set()`]: crate::TransformTo::into_set
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::{HashSet, LinkedList};
  ///
  /// let a = LinkedList::from([1, 2, 3]);
  ///
  /// assert_eq!(a.to_set(), HashSet::from([1, 2, 3]));
  /// ```
  fn to_set(&self) -> HashSet<Item>
  where
    Item: Eq + Hash + Clone;
}
