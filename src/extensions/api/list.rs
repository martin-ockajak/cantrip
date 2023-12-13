/// The `Functor` trait represents an ability to map over parametric types with single type parameter.
///
/// # Type Parameters
///
/// * `A` - type parameter of the implementing type
/// * `R` - type parameter of the resulting type after mapping
pub trait ListFunctor<A, B> {
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
  fn map(&self, function: impl Fn(&A) -> B) -> Self::C<B>;
}

pub trait ListMonad<A, B> {
  type C<X>;

  fn unit(value: A) -> Self::C<A>
    where
      A: Clone;

  fn flat_map<R>(&self, function: impl Fn(&A) -> R) -> Self::C<B>
    where
      R: IntoIterator<Item = B> + Clone;
}

pub trait ListCollection<A: Clone> {
  type C<X>;

  fn delete(&self, value: &A) -> Self
    where
      A: PartialEq; fn add(&self, value: A) -> Self;

  fn diff(&self, iterable: &(impl IntoIterator<Item = A> + Clone)) -> Self
    where
      A: PartialEq;

  fn filter(&self, predicate: impl Fn(&A) -> bool) -> Self;

  fn filter_map<B>(&self, function: impl Fn(&A) -> Option<B>) -> Self::C<B>;

  fn find_map<B>(&self, function: impl Fn(&A) -> Option<B>) -> Option<B>;


  fn merge(&self, iterable: &(impl IntoIterator<Item = A> + Clone)) -> Self;
}

pub trait Ordered<A: Clone> {
  type C<X>;

  fn enumerate(&self) -> Self::C<(usize, A)>;

  fn map_while<B>(&self, predicate: impl Fn(&A) -> Option<B>) -> Self::C<B>;

  fn partition(&self, predicate: impl Fn(&A) -> bool) -> (Self, Self) where Self: Sized;

  fn repeat(&self, n: usize) -> Self;

  fn skip(&self, n: usize) -> Self;

  fn take(&self, n: usize) -> Self;

  fn zip<I>(&self, iterable: &I) -> Self::C<(A, I::Item)>
    where
      I: IntoIterator + Clone;
}
