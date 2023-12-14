pub trait Ordered<A> {
  fn head(&self) -> Option<&A>;

  fn last(&self) -> Option<&A>;

  fn position(&self, predicate: impl FnMut(&A) -> bool) -> Option<usize>;

  fn rfind(&self, predicate: impl FnMut(&A) -> bool) -> Option<&A>;
}
