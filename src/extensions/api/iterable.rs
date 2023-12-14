pub trait Iterable<A> {
  fn all(&self, predicate: impl FnMut(&A) -> bool) -> bool;

  fn any(&self, predicate: impl FnMut(&A) -> bool) -> bool;

  fn find(&self, predicate: impl FnMut(&A) -> bool) -> Option<&A>;

  fn fold<B>(&self, init: B, function: impl FnMut(B, &A) -> B) -> B;

  fn reduce(&self, function: impl FnMut(&A, &A) -> A) -> Option<A>;

  fn rfold<B>(&self, init: B, function: impl FnMut(B, &A) -> B) -> B;
}
