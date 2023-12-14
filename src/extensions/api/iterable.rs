pub trait Iterable<A> {
  fn all(&self, predicate: impl Fn(&A) -> bool) -> bool;

  fn any(&self, predicate: impl Fn(&A) -> bool) -> bool;

  fn find(&self, predicate: impl Fn(&A) -> bool) -> Option<&A>;

  fn fold<B>(&self, init: B, function: impl Fn(B, &A) -> B) -> B;

  fn reduce(&self, function: impl Fn(&A, &A) -> A) -> Option<A>;

  fn rfold<B>(&self, init: B, function: impl Fn(B, &A) -> B) -> B;
}

pub trait Ordered<A> {
  fn head(&self) -> Option<&A>;

  fn last(&self) -> Option<&A>;

  fn position(&self, predicate: impl Fn(&A) -> bool) -> Option<usize>;

  fn rfind(&self, predicate: impl Fn(&A) -> bool) -> Option<&A>;
}
