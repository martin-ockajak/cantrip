use std::hash::Hash;

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
  fn map(&self, function: impl Fn(&A) -> R) -> Self::C<R>;
}

pub trait EqFunctor<A: Eq + Hash, R: Eq + Hash> {
  type C<X>;

  fn map(&self, function: impl Fn(&A) -> R) -> Self::C<R>;
}

pub trait Monad<A, R> {
  type C<X>;

  fn unit(value: A) -> Self::C<A>
    where
      A: Clone;

  fn flat_map(&self, function: impl Fn(&A) -> Self::C<R>) -> Self::C<R>;
}

pub trait EqMonad<A: Eq + Hash, R: Eq + Hash> {
  type C<X>;

  fn unit(value: A) -> Self::C<A>
    where
      A: Clone;

  fn flat_map(&self, function: impl Fn(&A) -> Self::C<R>) -> Self::C<R>;
}

pub trait AggregateIterable<A> {
  fn all(&self, predicate: impl Fn(&A) -> bool) -> bool;

  fn any(&self, predicate: impl Fn(&A) -> bool) -> bool;

  fn find(&self, predicate: impl Fn(&A) -> bool) -> Option<&A>
    where
      A: Clone;

  fn fold<B>(&self, init: B, function: impl Fn(B, &A) -> B) -> B;

  fn rfold<B>(&self, init: B, function: impl Fn(B, &A) -> B) -> B;
}

pub trait Iterable<A: Clone> {
  type C<X>;

  fn zip<I>(&self, iterable: &I) -> Self::C<(A, I::Item)>
    where
      I: IntoIterator + Clone;

  fn zip_with_index(&self) -> Self::C<(A, usize)>;

  fn filter(&self, predicate: impl Fn(&A) -> bool) -> Self;
}

pub trait EqIterable<A: Eq + Hash + Clone> {
  type C<X>;

  fn filter(&self, predicate: impl Fn(&A) -> bool) -> Self;
}

pub trait Collection<A: Clone> {
  fn add(&self, value: A) -> Self;

  fn add_all(&self, iterable: &(impl IntoIterator<Item = A> + Clone)) -> Self;

  fn remove(&self, value: A) -> Self
    where
      A: PartialEq;

  fn remove_all(&self, iterable: &(impl IntoIterator<Item = A> + Clone)) -> Self
    where
      A: PartialEq;
}
