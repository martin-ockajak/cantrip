pub trait Slice<A> {
  fn init(&self) -> &Self;

  fn skip(&self, n: usize) -> &Self;

  fn skip_while(&self, predicate: impl FnMut(&A) -> bool) -> &Self;

  fn tail(&self) -> &Self;
  //
  // fn take(&self, n: usize) -> &Self;

  fn take_while(&self, predicate: impl FnMut(&A) -> bool) -> &Self;
}
