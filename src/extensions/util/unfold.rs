pub(crate) fn unfold<A, S, F>(init: S, function: F) -> Unfold<S, F>
where
  F: FnMut(&mut S) -> Option<A>,
{
  Unfold { state: init, function }
}

impl<A, S, F> Iterator for Unfold<S, F>
where
  F: FnMut(&mut S) -> Option<A>,
{
  type Item = A;

  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    (self.function)(&mut self.state)
  }
}

pub(crate) struct Unfold<S, F> {
  pub state: S,
  function: F,
}
