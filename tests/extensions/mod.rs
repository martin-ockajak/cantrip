use cantrip::extensions::*;

pub fn test_iterable<A, C>(data: C, mut predicate: impl FnMut(&A) -> bool) -> bool
where
  C: Iterable<A> + IntoIterator<Item = A> + Clone,
{
  data.all(|x| predicate(x)) == data.clone().into_iter().all(|x| predicate(&x))
}

pub fn test_ordered<A, C>(data: C, mut predicate: impl FnMut(&A) -> bool) -> bool
  where
    C: Ordered<A> + IntoIterator<Item = A> + Clone,
{
  data.position(|x| predicate(x)) == data.clone().into_iter().position(|x| predicate(&x))
}
//
// pub fn test_list<A, C>(data: C, mut predicate: impl FnMut(&A) -> bool) -> bool
//   where
//     C: List<A> + IntoIterator<Item = A> + Clone + PartialEq,
// {
//   data.clone().rev() == data.clone().into_iter().rev().collect()
// }
