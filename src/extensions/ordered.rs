pub trait Ordered<Item> {
  fn position(&self, predicate: impl FnMut(&Item) -> bool) -> Option<usize>;

  fn rfind(&self, predicate: impl FnMut(&Item) -> bool) -> Option<&Item>;

  fn rfold<B>(&self, init: B, function: impl FnMut(B, &Item) -> B) -> B;

  fn rposition(&self, predicate: impl FnMut(&Item) -> bool) -> Option<usize>;
}
