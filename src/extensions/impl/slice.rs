use crate::extensions::api::iterable::Iterable;
use crate::extensions::Ordered;

impl<A> Iterable<A> for [A] {
  fn all(&self, predicate: impl Fn(&A) -> bool) -> bool {
    self.iter().all(predicate)
  }

  fn any(&self, predicate: impl Fn(&A) -> bool) -> bool {
    self.iter().any(predicate)
  }

  fn find(&self, predicate: impl Fn(&A) -> bool) -> Option<&A> {
    self.iter().find(|&x| predicate(x))
  }

  fn fold<B>(&self, init: B, function: impl Fn(B, &A) -> B) -> B {
    self.iter().fold(init, function)
  }

  fn reduce(&self, function: impl Fn(&A, &A) -> A) -> Option<A> where A: Clone {
    let mut iterator = self.iter();
    iterator.next().and_then(|init| Some(iterator.fold(init.clone(), |r, x| function(&r, x))))
  }

  fn rfold<B>(&self, init: B, function: impl Fn(B, &A) -> B) -> B {
    self.iter().rfold(init, function)
  }
}

impl<A> Ordered<A> for [A] {
  fn position(&self, predicate: impl Fn(&A) -> bool) -> Option<usize> {
    self.iter().position(predicate)
  }

  fn rfind(&self, predicate: impl Fn(&A) -> bool) -> Option<&A> {
    self.iter().rev().find(|&x| predicate(x))
  }
}

#[cfg(test)]
mod tests {
}
