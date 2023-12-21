use crate::extensions::util::append::Append;
use std::collections::HashMap;
use std::hash::Hash;
use std::iter;
use crate::extensions::util::multi_map::MultiMap;

pub trait Sequence<Item> {
  type This<I>;

  fn chunked(self, chunk_size: usize) -> Self::This<Self>
  where
    Self: IntoIterator<Item = Item> + Sized + Default + Extend<Item>;

  #[inline]
  fn delete(self, index: usize) -> Self
  where
    Self: IntoIterator<Item = Item> + Sized + FromIterator<Item>,
  {
    self.into_iter().enumerate().filter_map(|(i, x)| if i != index { Some(x) } else { None }).collect()
  }

  #[inline]
  fn enumerate(self) -> Self::This<(usize, Item)>
  where
    Self: IntoIterator<Item = Item> + Sized,
    Self::This<(usize, Item)>: FromIterator<(usize, Item)>,
  {
    self.into_iter().enumerate().collect()
  }

  #[inline]
  fn grouped_by<K: Eq + Hash>(self, mut to_key: impl FnMut(&Item) -> K) -> HashMap<K, Self>
    where
      Self: IntoIterator<Item = Item> + Sized + Default + Extend<Item>,
  {
    HashMap::group_pairs(self.into_iter().map(|x| (to_key(&x), x)))
  }

  fn init(self) -> Self;

  fn interleave(self, iterable: impl IntoIterator<Item = Item>) -> Self
  where
    Self: Default + Extend<Item>;

  // FIXME - decide if the lifetime declarations are worth it
  // fn map<'c, B>(&'c self, function: impl FnMut(&Item) -> B) -> Self::This<B>
  //   where
  //     Item: 'c,
  //     Self: Iterable<Item<'c> = &'c Item> + 'c,
  //     Self::This<B>: FromIterator<B>,
  // {
  //   self.iterator().map(function).collect()
  // }

  fn map_while<B>(&self, predicate: impl FnMut(&Item) -> Option<B>) -> Self::This<B>;

  #[inline]
  fn merge(self, iterable: impl IntoIterator<Item = Item>) -> Self
  where
    Self: IntoIterator<Item = Item> + Sized + FromIterator<Item>,
  {
    self.into_iter().chain(iterable.into_iter()).collect()
  }

  fn position(&self, predicate: impl FnMut(&Item) -> bool) -> Option<usize>;

  fn rev(self) -> Self;

  fn scan<S, B>(&self, init: S, function: impl FnMut(&mut S, &Item) -> Option<B>) -> Self::This<B>;

  #[inline]
  fn skip(self, n: usize) -> Self
  where
    Self: IntoIterator<Item = Item> + Sized + FromIterator<Item>,
  {
    self.into_iter().skip(n).collect()
  }

  #[inline]
  fn skip_while(self, predicate: impl FnMut(&Item) -> bool) -> Self
  where
    Self: IntoIterator<Item = Item> + Sized + FromIterator<Item>,
  {
    self.into_iter().skip_while(predicate).collect()
  }

  #[inline]
  fn step_by(self, step: usize) -> Self
  where
    Self: IntoIterator<Item = Item> + Sized + FromIterator<Item>,
  {
    self.into_iter().step_by(step).collect()
  }

  #[inline]
  fn tail(self) -> Self
  where
    Self: IntoIterator<Item = Item> + Sized + FromIterator<Item>,
  {
    self.into_iter().skip(1).collect()
  }

  #[inline]
  fn take(self, n: usize) -> Self
  where
    Self: IntoIterator<Item = Item> + Sized + FromIterator<Item>,
  {
    self.into_iter().take(n).collect()
  }

  #[inline]
  fn take_while(self, predicate: impl FnMut(&Item) -> bool) -> Self
  where
    Self: IntoIterator<Item = Item> + Sized + FromIterator<Item>,
  {
    self.into_iter().take_while(predicate).collect()
  }

  #[inline]
  fn unit(value: Item) -> Self
  where
    Self: FromIterator<Item>,
  {
    iter::once(value).collect()
  }

  #[inline]
  fn unzip<B, C>(self) -> (Self::This<B>, Self::This<C>)
  where
    Self: IntoIterator<Item = (B, C)> + Sized,
    Self::This<B>: Default + Extend<B>,
    Self::This<C>: Default + Extend<C>,
  {
    self.into_iter().unzip()
  }

  #[inline]
  fn zip<I: IntoIterator>(self, iterable: I) -> Self::This<(Item, I::Item)>
  where
    Self: IntoIterator<Item = Item> + Sized,
    Self::This<(Item, I::Item)>: FromIterator<(Item, I::Item)>,
  {
    self.into_iter().zip(iterable).collect()
  }
}

#[inline]
pub(crate) fn chunked<Item, Chunk, Result>(iterator: impl Iterator<Item = Item>, chunk_size: usize) -> Result
where
  Chunk: IntoIterator<Item = Item> + Sized + Default + Append<Item>,
  Result: Default + Append<Chunk>,
{
  let mut result = Result::default();
  let mut chunk = Chunk::default();
  let mut index: usize = 0;
  let mut chunk_index: usize = 0;
  for item in iterator {
    chunk.append(item);
    index += 1;
    chunk_index += 1;
    if chunk_index == chunk_size {
      result.append(chunk);
      chunk = Chunk::default();
      chunk_index = 0;
    }
  }
  if index > 0 && chunk_index == 0 {
    result.append(chunk);
  }
  result
}

#[inline]
pub(crate) fn init<Item, Result, Iterable>(iterator: Iterable) -> Result
where
  Iterable: Iterator<Item = Item> + ExactSizeIterator,
  Result: Sized + FromIterator<Item>,
{
  let size = iterator.len() - 1;
  iterator.skip(size).collect()
}

#[inline]
pub(crate) fn interleave<Item, Result>(
  iterator: impl Iterator<Item = Item>, iterable: impl IntoIterator<Item = Item>,
) -> Result
where
  Result: Default + Append<Item>,
{
  let mut result = Result::default();
  for (item1, item2) in iterator.zip(iterable) {
    result.append(item1);
    result.append(item2);
  }
  result
}
