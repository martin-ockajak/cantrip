use crate::extensions::MultiMap;
use std::hash::Hash;

pub trait List<A> {
  type Root<X>;

  fn add(self, value: A) -> Self;

  fn concat(self, iterable: impl IntoIterator<Item = A>) -> Self;

  fn delete(self, value: &A) -> Self
  where
    A: PartialEq;

  fn diff(self, iterable: impl IntoIterator<Item = A>) -> Self
  where
    A: Eq + Hash;

  fn distinct(self) -> Self
  where
    A: Eq + Hash;

  fn enumerate(self) -> Self::Root<(usize, A)>;

  fn filter(self, predicate: impl FnMut(&A) -> bool) -> Self;

  fn filter_map<B>(&self, function: impl FnMut(&A) -> Option<B>) -> Self::Root<B>;

  fn find_map<B>(&self, function: impl FnMut(&A) -> Option<B>) -> Option<B>;

  fn flat_map<B, R>(&self, function: impl FnMut(&A) -> R) -> Self::Root<B>
  where
    R: IntoIterator<Item = B>;

  fn flatten<B>(self) -> Self::Root<B>
  where
    A: IntoIterator<Item = B>;

  fn group_by<K, M>(self, group_key: impl FnMut(&A) -> K) -> M
    where
      K: Eq + Hash,
      M: MultiMap<K, Self::Root<A>>;

  fn init(self) -> Self;

  fn intersect(self, iterable: impl IntoIterator<Item = A>) -> Self
  where
    A: Eq + Hash;

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
  /// // let result: Vec<i32> = vec![1, 2, 3].map(|x| x + 1);
  /// ```
  fn map<B>(&self, function: impl FnMut(&A) -> B) -> Self::Root<B>;

  fn map_while<B>(&self, predicate: impl FnMut(&A) -> Option<B>) -> Self::Root<B>;

  fn partition(self, predicate: impl FnMut(&A) -> bool) -> (Self, Self)
  where
    Self: Sized;

  fn rev(self) -> Self;

  fn scan<S, B>(&self, init: S, function: impl FnMut(&mut S, &A) -> Option<B>) -> Self::Root<B>;

  fn skip(self, n: usize) -> Self;

  fn skip_while(self, predicate: impl FnMut(&A) -> bool) -> Self;

  fn tail(self) -> Self;

  fn take(self, n: usize) -> Self;

  fn take_while(self, predicate: impl FnMut(&A) -> bool) -> Self;

  fn unit(value: A) -> Self;

  fn unzip<B, C, FromB, FromC>(self) -> (Self::Root<B>, Self::Root<C>)
  where
    Self: IntoIterator<Item = (B, C)>;

  fn zip<I>(self, iterable: I) -> Self::Root<(A, I::Item)>
  where
    I: IntoIterator;
}
