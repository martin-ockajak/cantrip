// use std::mem;
//
// pub(crate) fn iterate<S, F>(init: S, function: F) -> Iterate<S, F>
// where
//   F: FnMut(&S) -> S,
// {
//   Iterate { state: init, function }
// }
//
// impl<S, F> Iterator for Iterate<S, F>
// where
//   F: FnMut(&S) -> S,
// {
//   type Item = S;
//
//   #[inline]
//   fn next(&mut self) -> Option<Self::Item> {
//     let state = (self.function)(&self.state);
//     Some(mem::replace(&mut self.state, state))
//   }
//
//   #[inline]
//   fn size_hint(&self) -> (usize, Option<usize>) {
//     (usize::max_value(), None)
//   }
// }
//
// pub(crate) struct Iterate<S, F> {
//   state: S,
//   function: F,
// }
