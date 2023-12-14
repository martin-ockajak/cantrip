use std::hash::Hash;

/// The `Functor` trait represents an ability to map over parametric types with single type parameter.
///
/// # Type Parameters
///
/// * `A` - type parameter of the implementing type
pub trait ListFunctor<A> {
  type C<X>;

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
  fn map<B>(&self, function: impl FnMut(&A) -> B) -> Self::C<B>;
}

pub trait ListMonad<A> {
  type C<X>;

  fn unit(value: A) -> Self::C<A>;

  fn flat_map<B, R>(&self, function: impl FnMut(&A) -> R) -> Self::C<B>
  where
    R: IntoIterator<Item = B>;
}

pub trait ListOps<A> {
  type C<X>;

  fn add(self, value: A) -> Self;

  fn delete(self, value: &A) -> Self
  where
    A: PartialEq;

  fn diff(self, iterable: impl IntoIterator<Item = A>) -> Self
  where
    A: Eq + Hash;

  fn enumerate(self) -> Self::C<(usize, A)>;

  fn filter(self, predicate: impl FnMut(&A) -> bool) -> Self;

  fn filter_map<B>(&self, function: impl FnMut(&A) -> Option<B>) -> Self::C<B>;

  fn find_map<B>(&self, function: impl FnMut(&A) -> Option<B>) -> Option<B>;

  fn merge(self, iterable: impl IntoIterator<Item = A>) -> Self;

  fn map_while<B>(&self, predicate: impl FnMut(&A) -> Option<B>) -> Self::C<B>;

  fn partition(self, predicate: impl FnMut(&A) -> bool) -> (Self, Self)
  where
    Self: Sized;

  fn skip(self, n: usize) -> Self;

  fn take(self, n: usize) -> Self;

  fn zip<I>(self, iterable: I) -> Self::C<(A, I::Item)>
  where
    I: IntoIterator;
}
