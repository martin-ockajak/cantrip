pub trait SliceOps<A> {
  fn init(&self) -> &Self;

  fn tail(&self) -> &Self;
}
