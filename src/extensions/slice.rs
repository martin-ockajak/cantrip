pub trait Slice<A> {
  fn init(&self) -> &Self;

  fn skip_while(&self, predicate: impl FnMut(&A) -> bool) -> &Self;

  fn tail(&self) -> &Self;

  fn take_while(&self, predicate: impl FnMut(&A) -> bool) -> &Self;
}
