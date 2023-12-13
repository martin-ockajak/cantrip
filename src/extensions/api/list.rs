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
