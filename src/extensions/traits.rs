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
  /// The resulting values are collected into a new container of the same type.
  ///
  /// # Arguments
  ///
  /// * `self` - the container to apply the mapping to.
  /// * `f` - the closure to apply to each element.
  ///
  /// # Returns
  ///
  /// A new container of the same type, containing the mapped values.
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
  fn all<P>(&self, predicate: P) -> bool
    where
      P: Fn(&A) -> bool;

  fn any<P>(&self, predicate: P) -> bool
    where
      P: Fn(&A) -> bool;

  fn filter<P>(&self, predicate: P) -> Self
    where
      P: Fn(&A) -> bool,
      A: Clone;

  fn fold<B, F>(&self, init: B, function: F) -> B
    where
      F: Fn(B, &A) -> B;
}

pub trait Collection<A: Clone> {
  fn add(&self, value: A) -> Self;

  fn add_seq<I>(&self, values: &I) -> Self
    where
      I: Clone + IntoIterator<Item = A>;

  fn remove(&self, value: A) -> Self
    where
      A: PartialEq;

  fn remove_seq<I>(&self, values: &I) -> Self
    where
      A: PartialEq,
      I: Clone + IntoIterator<Item = A>;
}
