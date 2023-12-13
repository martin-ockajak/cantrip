
pub trait Iterable<A> {
  fn all(&self, predicate: impl Fn(&A) -> bool) -> bool;

  fn any(&self, predicate: impl Fn(&A) -> bool) -> bool;

  fn find(&self, predicate: impl Fn(&A) -> bool) -> Option<&A>
    where
      A: Clone;

  fn fold<B>(&self, init: B, function: impl Fn(B, &A) -> B) -> B;

  fn reduce(&self, function: impl Fn(&A, &A) -> A) -> Option<A>
    where
      A: Clone;

  fn rfold<B>(&self, init: B, function: impl Fn(B, &A) -> B) -> B;
}
