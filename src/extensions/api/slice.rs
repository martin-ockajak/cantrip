pub trait Slice<A> {
  fn init(&self) -> &Self;

  fn tail(&self) -> &Self;
}
