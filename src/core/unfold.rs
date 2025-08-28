pub(crate) fn unfold<A, F>(function: F) -> Unfold<F>
where F: FnMut() -> Option<A> {
  Unfold { function }
}

pub(crate) struct Unfold<F> {
  function: F,
}

impl<A, F> Iterator for Unfold<F>
where F: FnMut() -> Option<A>
{
  type Item = A;

  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    (self.function)()
  }
}
