pub trait Slice<Item> {
  fn init(&self) -> &Self;

  fn skip_while(&self, predicate: impl FnMut(&Item) -> bool) -> &Self;

  fn tail(&self) -> &Self;

  fn take_while(&self, predicate: impl FnMut(&Item) -> bool) -> &Self;
}
