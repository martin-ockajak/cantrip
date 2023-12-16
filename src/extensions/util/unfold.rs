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

// pub trait Kind1 {
//   type Param1;
// }
//
// impl<A1> Kind1 for Option<A1> {
//   type Param1 = A1;
// }
//
// pub trait Apply1<A1> {
//   type Param1;
//   type Result;
// }
//
// impl<A1, P1> Apply1<A1> for Option<P1> {
//   type Param1 = A1;
//   type Result = Option<A1>;
// }
