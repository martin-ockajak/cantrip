use crate::extensions::util::append::Append;
use std::collections::HashMap;
use std::hash::Hash;
use std::iter;

pub trait Sequence<Item> {
  type This<I>;

  fn add(self, value: Item) -> Self
  where
    Self: IntoIterator<Item = Item> + Sized + FromIterator<Item>,
  {
    self.into_iter().chain(iter::once(value)).collect()
  }

  fn chunked(self, chunk_size: usize) -> Self::This<Self>
  where
    Self: IntoIterator<Item = Item> + Sized;

  fn delete(self, index: usize) -> Self
  where
    Self: IntoIterator<Item = Item> + Sized + FromIterator<Item>,
  {
    self.into_iter().enumerate().filter_map(|(i, x)| if i != index { Some(x) } else { None }).collect()
  }

  fn enumerate(self) -> Self::This<(usize, Item)>
  where
    Self: IntoIterator<Item = Item> + Sized,
    Self::This<(usize, Item)>: FromIterator<(usize, Item)>,
  {
    self.into_iter().enumerate().collect()
  }

  fn exclude(self, value: &Item) -> Self
  where
    Item: PartialEq,
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    let mut removed = false;
    self
      .into_iter()
      .filter(|x| {
        if removed {
          true
        } else {
          removed = true;
          value != x
        }
      })
      .collect()
  }

  fn filter(self, predicate: impl FnMut(&Item) -> bool) -> Self
  where
    Self: IntoIterator<Item = Item> + Sized + FromIterator<Item>,
  {
    self.into_iter().filter(predicate).collect()
  }

  fn filter_map<B>(&self, function: impl FnMut(&Item) -> Option<B>) -> Self::This<B>;

  fn find_map<B>(&self, function: impl FnMut(&Item) -> Option<B>) -> Option<B>;

  fn flat_map<B, R>(&self, function: impl FnMut(&Item) -> R) -> Self::This<B>
  where
    R: IntoIterator<Item = B>;

  fn flat<B>(self) -> Self::This<B>
  where
    Item: IntoIterator<Item = B>,
    Self: IntoIterator<Item = Item> + Sized,
    Self::This<B>: FromIterator<B>,
  {
    self.into_iter().flatten().collect()
  }

  fn grouped_by<K>(self, to_key: impl FnMut(&Item) -> K) -> HashMap<K, Self>
  where
    K: Eq + Hash,
    Self: Sized;

  fn init(self) -> Self;

  fn interleave(self, iterable: impl IntoIterator<Item = Item>) -> Self;

  /// Applies the given closure `f` to each element in the container.
  ///
  /// The closure `f` takes a reference to an element of type `A` and returns a value of type `R`.
  /// The resulting other are collected into a new container of the same type.
  ///
  /// # Arguments
  ///
  /// * `self` - the container to apply the mapping to.
  /// * `f` - the closure to apply to each element.
  ///
  /// # Returns
  ///
  /// A new container of the same type, containing the mapped other.
  ///
  /// # Type Parameters
  ///
  /// * `F` - type of the closure, which takes a reference to an element of type `A` and returns a value of type `B`.
  ///
  /// # Constraints
  ///
  /// * `F: FnMut(&A) -> B` - the closure must be callable with a reference to an element of type `A` and return a value of type `B`.
  ///
  /// # Safety
  ///
  /// The caller must ensure that the closure does not mutate any shared state while being executed.
  /// The closure must not panic while being executed, as this will lead to undefined behavior.
  ///
  /// # Examples
  ///
  /// ```
  /// use crate::cantrip::extensions::*;
  ///
  /// let result: Vec<i32> = vec![1, 2, 3].map(|x| x + 1);
  /// ```
  fn map<B>(&self, function: impl FnMut(&Item) -> B) -> Self::This<B>;

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

  fn merge(self, iterable: impl IntoIterator<Item = Item>) -> Self
  where
    Self: IntoIterator<Item = Item> + Sized + FromIterator<Item>,
  {
    self.into_iter().chain(iterable.into_iter()).collect()
  }

  fn partition(self, predicate: impl FnMut(&Item) -> bool) -> (Self, Self)
  where
    Self: Sized + Default + Extend<Item> + IntoIterator<Item = Item> + Sized + FromIterator<Item>,
  {
    self.into_iter().partition(predicate)
  }

  fn rev(self) -> Self;

  fn scan<S, B>(&self, init: S, function: impl FnMut(&mut S, &Item) -> Option<B>) -> Self::This<B>;

  fn skip(self, n: usize) -> Self
  where
    Self: IntoIterator<Item = Item> + Sized + FromIterator<Item>,
  {
    self.into_iter().skip(n).collect()
  }

  fn skip_while(self, predicate: impl FnMut(&Item) -> bool) -> Self
  where
    Self: IntoIterator<Item = Item> + Sized + FromIterator<Item>,
  {
    self.into_iter().skip_while(predicate).collect()
  }

  fn step_by(self, step: usize) -> Self
  where
    Self: IntoIterator<Item = Item> + Sized + FromIterator<Item>,
  {
    self.into_iter().step_by(step).collect()
  }

  fn tail(self) -> Self
  where
    Self: IntoIterator<Item = Item> + Sized + FromIterator<Item>,
  {
    let mut iterator = self.into_iter();
    iterator.next();
    iterator.collect()
  }

  fn take(self, n: usize) -> Self
  where
    Self: IntoIterator<Item = Item> + Sized + FromIterator<Item>,
  {
    self.into_iter().take(n).collect()
  }

  fn take_while(self, predicate: impl FnMut(&Item) -> bool) -> Self
  where
    Self: IntoIterator<Item = Item> + Sized + FromIterator<Item>,
  {
    self.into_iter().take_while(predicate).collect()
  }

  fn unit(value: Item) -> Self
  where
    Self: FromIterator<Item>,
  {
    iter::once(value).collect()
  }

  fn unzip<B, C>(self) -> (Self::This<B>, Self::This<C>)
  where
    Self: IntoIterator<Item = (B, C)> + Sized,
    Self::This<B>: Default + Extend<B>,
    Self::This<C>: Default + Extend<C>,
  {
    self.into_iter().unzip()
  }

  fn zip<I>(self, iterable: I) -> Self::This<(Item, I::Item)>
  where
    I: IntoIterator,
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
