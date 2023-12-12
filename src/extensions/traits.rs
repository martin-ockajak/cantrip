use std::iter::Cycle;
use std::slice::Iter;

/// The `Functor` trait represents an ability to map over parametric types with single type parameter.
///
/// # Type Parameters
///
/// * `A` - type parameter of the implementing type
/// * `R` - type parameter of the resulting type after mapping
pub trait Functor<A, R> {
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
  /// * `F` - type of the closure, which takes a reference to an element of type `A` and returns a value of type `R`.
  ///
  /// # Constraints
  ///
  /// * `F: Fn(&A) -> R` - the closure must be callable with a reference to an element of type `A` and return a value of type `R`.
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
  fn map<F>(&self, function: F) -> Self::C<R>
    where
      F: Fn(&A) -> R;
}

pub trait Iterable<A> {
  type C<X>;

  fn all<P>(&self, predicate: P) -> bool
    where
      P: Fn(&A) -> bool;

  fn any<P>(&self, predicate: P) -> bool
    where
      P: Fn(&A) -> bool;

  fn cycle(&self) -> Cycle<Iter<A>>;

  fn zip<I>(&self, other: &I) -> Self::C<(A, I::Item)>
    where
      I: Clone + IntoIterator,
      A: Clone;

  fn zip_with_index(&self) -> Self::C<(A, usize)>
    where
      A: Clone;

  fn filter<P>(&self, predicate: P) -> Self
    where
      P: Fn(&A) -> bool,
      A: Clone;

  fn find<P>(&self, predicate: P) -> Option<&A>
    where
      P: Fn(&A) -> bool,
      A: Clone;

  fn fold<B, F>(&self, init: B, function: F) -> B
    where
      F: Fn(B, &A) -> B;

  fn rfold<B, F>(&self, init: B, function: F) -> B
    where
      F: Fn(B, &A) -> B;
}

pub trait Collection<A: Clone> {
  fn add(&self, value: A) -> Self;

  fn add_seq<I>(&self, other: &I) -> Self
    where
      I: Clone + IntoIterator<Item = A>;

  fn remove(&self, value: A) -> Self
    where
      A: PartialEq;

  fn remove_seq<I>(&self, other: &I) -> Self
    where
      I: Clone + IntoIterator<Item = A>,
      A: PartialEq;
}
