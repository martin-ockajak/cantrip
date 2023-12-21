use std::collections::HashSet;
use std::hash::Hash;
use std::iter;
use std::iter::{Product, Sum};

pub trait Collectible<Item>: IntoIterator<Item = Item> + Sized {
  type This<I>;

  fn add(self, value: Item) -> Self
  where
    Self: IntoIterator<Item = Item> + Sized + FromIterator<Item>,
  {
    self.into_iter().chain(iter::once(value)).collect()
  }

  #[inline]
  fn diff(self, iterable: impl IntoIterator<Item = Item>) -> Self
  where
    Item: Eq + Hash,
    Self: FromIterator<Item>,
  {
    let mut removed: HashSet<Item> = HashSet::new();
    removed.extend(iterable);
    self.into_iter().filter(|x| !removed.contains(x)).collect()
  }

  #[inline]
  fn exclude(self, value: &Item) -> Self
  where
    Item: PartialEq,
    Self: IntoIterator<Item = Item> + Sized + FromIterator<Item>,
  {
    let mut removed = false;
    self
      .into_iter()
      .filter(|x| {
        if removed {
          true
        } else {
          removed = true;
          value != x
        }
      })
      .collect()
  }

  #[inline]
  fn filter(self, predicate: impl FnMut(&Item) -> bool) -> Self
  where
    Self: IntoIterator<Item = Item> + Sized + FromIterator<Item>,
  {
    self.into_iter().filter(predicate).collect()
  }

  #[inline]
  fn flat<B>(self) -> Self::This<B>
  where
    Item: IntoIterator<Item = B>,
    Self: IntoIterator<Item = Item> + Sized,
    Self::This<B>: FromIterator<B>,
  {
    self.into_iter().flatten().collect()
  }

  #[inline]
  fn intersect(self, iterable: impl IntoIterator<Item = Item>) -> Self
  where
    Item: Eq + Hash,
    Self: FromIterator<Item>,
  {
    let mut retained: HashSet<Item> = HashSet::new();
    retained.extend(iterable);
    self.into_iter().filter(|x| retained.contains(x)).collect()
  }

  /// Applies the given closure `f` to each element in the container.
  ///
  /// The closure `f` takes a reference to an element of type `A` and returns a value of type `R`.
  /// The resulting other are collected into a new container of the same type.
  ///
  /// # Arguments
  ///
  /// * `self` - the container to apply the mapping to.
  /// * `f` - the closure to apply to each element.
  ///
  /// # Returns
  ///
  /// A new container of the same type, containing the mapped other.
  ///
  /// # Type Parameters
  ///
  /// * `F` - type of the closure, which takes a reference to an element of type `A` and returns a value of type `B`.
  ///
  /// # Constraints
  ///
  /// * `F: FnMut(&A) -> B` - the closure must be callable with a reference to an element of type `A` and return a value of type `B`.
  ///
  /// # Safety
  ///
  /// The caller must ensure that the closure does not mutate any shared state while being executed.
  /// The closure must not panic while being executed, as this will lead to undefined behavior.
  ///
  /// # Examples
  ///
  /// ```
  /// use crate::cantrip::extensions::*;
  ///
  /// let result: Vec<i32> = vec![1, 2, 3].map(|x| x + 1);
  /// ```
  fn map<B>(&self, function: impl FnMut(&Item) -> B) -> Self::This<B>
  where
    Self::This<B>: FromIterator<B>;

  // fn largest_by(self, n: usize, compare: impl FnMut(&Item, &Item) -> Ordering) -> Self
  // where
  //   Item: Ord,
  //   Self: IntoIterator<Item = Item> + Sized + Default + Extend<Item>,
  // {
  //   largest_by(self, n, compare)
  // }

  #[inline]
  fn merge(self, iterable: impl IntoIterator<Item = Item>) -> Self
  where
    Self: IntoIterator<Item = Item> + Sized + FromIterator<Item>,
  {
    self.into_iter().chain(iterable).collect()
  }

  #[inline]
  fn partition(self, predicate: impl FnMut(&Item) -> bool) -> (Self, Self)
  where
    Self: Sized + Default + Extend<Item> + IntoIterator<Item = Item> + Sized + FromIterator<Item>,
  {
    self.into_iter().partition(predicate)
  }

  #[inline]
  fn product(self) -> Item
  where
    Item: Product,
  {
    self.into_iter().product()
  }

  #[inline]
  fn reduce(self, function: impl FnMut(Item, Item) -> Item) -> Option<Item> {
    let mut iterator = self.into_iter();
    iterator.next().map(|result| iterator.fold(result, function))
  }

  #[inline]
  fn sum(self) -> Item
  where
    Item: Sum,
  {
    self.into_iter().sum()
  }

  #[inline]
  fn unit(value: Item) -> Self
  where
    Self: FromIterator<Item> + Sized,
  {
    iter::once(value).collect()
  }
}

// fn largest_by<Item, Collection>(
//   collection: Collection, n: usize, compare: impl FnMut(&Item, &Item) -> Ordering,
// ) -> Collection
// where
//   Item: Ord,
//   Collection: IntoIterator<Item = Item> + Sized + Default + Extend<Item>,
// {
//   let heap = BinaryHeap::new_by(compare);
//   heap.extend(collection);
//   let mut result = Collection::default();
//   for _ in 0..n {
//     result.extend(iter::once(heap.pop().unwrap()));
//   }
//   result
// }
