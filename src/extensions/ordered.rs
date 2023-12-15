pub trait Ordered<A> {
  fn position(&self, predicate: impl FnMut(&A) -> bool) -> Option<usize>;

  fn rfind(&self, predicate: impl FnMut(&A) -> bool) -> Option<&A>;

  fn rfold<B>(&self, init: B, function: impl FnMut(B, &A) -> B) -> B;

  fn rposition(&self, predicate: impl FnMut(&A) -> bool) -> Option<usize>;
}
