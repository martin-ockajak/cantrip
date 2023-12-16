use crate::extensions::Iterable;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::Hash;
use std::iter;
use std::ops::RangeBounds;

pub trait List<A> {
  type This<Item>;

  fn add(self, value: A) -> Self
  where
    Self: IntoIterator<Item = A> + Sized + FromIterator<A>,
  {
    self.into_iter().chain(iter::once(value)).collect()
  }

  fn concat(self, iterable: impl IntoIterator<Item = A>) -> Self
  where
    Self: IntoIterator<Item = A> + Sized + FromIterator<A>,
  {
    self.into_iter().chain(iterable.into_iter()).collect()
  }

  fn distinct(self) -> Self
  where
    A: Eq + Hash;

  fn distinct_by<K>(self, to_key: impl FnMut(&A) -> K) -> Self
  where
    K: Eq + Hash;

  fn delete(self, index: usize) -> Self
  where
    Self: IntoIterator<Item = A> + Sized + FromIterator<A>,
  {
    self.into_iter().enumerate().filter_map(|(i, x)| if i != index { Some(x) } else { None }).collect()
  }

  fn enumerate(self) -> Self::This<(usize, A)>
  where
    Self: IntoIterator<Item = A> + Sized,
    Self::This<(usize, A)>: FromIterator<(usize, A)>,
  {
    self.into_iter().enumerate().collect()
  }

  fn filter(self, predicate: impl FnMut(&A) -> bool) -> Self
  where
    Self: IntoIterator<Item = A> + Sized + FromIterator<A>,
  {
    self.into_iter().filter(predicate).collect()
  }

  fn exclude(self, value: &A) -> Self
  where
    A: PartialEq;

  fn filter_map<B>(&self, function: impl FnMut(&A) -> Option<B>) -> Self::This<B>;

  fn find_map<B>(&self, function: impl FnMut(&A) -> Option<B>) -> Option<B>;

  fn flat_map<B, R>(&self, function: impl FnMut(&A) -> R) -> Self::This<B>
  where
    R: IntoIterator<Item = B>;

  fn flatten<B>(self) -> Self::This<B>
  where
    A: IntoIterator<Item = B>,
    Self: IntoIterator<Item = A> + Sized,
    Self::This<B>: FromIterator<B>,
  {
    self.into_iter().flatten().collect()
  }

  fn group_by<K>(self, to_key: impl FnMut(&A) -> K) -> HashMap<K, Self>
  where
    K: Eq + Hash,
    Self: Sized;

  fn init(self) -> Self;

  fn interleave(self, iterable: impl IntoIterator<Item = A>) -> Self;

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
  fn map<B>(&self, function: impl FnMut(&A) -> B) -> Self::This<B>;

  // FIXME - make the lifetimes work
  // fn x_map<B>(&self, function: impl FnMut(&A) -> B) -> Self::This<B>
  // where
  //   A: 'c,
  //   Self: Iterable<Item<'c> = &'c A> + 'c,
  //   Self::This<B>: FromIterator<B>,
  // {
  //   let x = self.iterator();
  //   x.map(function).collect()
  // }

  fn map_while<B>(&self, predicate: impl FnMut(&A) -> Option<B>) -> Self::This<B>;

  fn partition(self, predicate: impl FnMut(&A) -> bool) -> (Self, Self)
  where
    Self: Sized + Default + Extend<A> + IntoIterator<Item = A> + Sized + FromIterator<A>,
  {
    self.into_iter().partition(predicate)
  }

  fn put(self, index: usize, element: A) -> Self
  where
    Self: IntoIterator<Item = A>;

  // FIXME - make the moving work
  // fn x_put(self, index: usize, element: A) -> Self
  // where
  //   Self: IntoIterator<Item = A> + Sized + FromIterator<A>,
  // {
  //   let mut iterator = self.into_iter();
  //   let mut value = Rc::new(element);
  //   unfold((0 as usize, false), |(current, done)| {
  //     if !*done && *current == index {
  //       *done = true;
  //       None
  //       // Rc::into_inner(value)
  //     } else {
  //       *current += 1;
  //       iterator.next()
  //     }
  //   })
  //   .collect()
  // }
  //
  fn replace(self, range: impl RangeBounds<usize>, replace_with: Self) -> Self
  where
    Self: IntoIterator<Item = A>;

  fn rev(self) -> Self;

  fn scan<S, B>(&self, init: S, function: impl FnMut(&mut S, &A) -> Option<B>) -> Self::This<B>;

  fn skip(self, n: usize) -> Self
  where
    Self: IntoIterator<Item = A> + Sized + FromIterator<A>,
  {
    self.into_iter().skip(n).collect()
  }

  fn skip_while(self, predicate: impl FnMut(&A) -> bool) -> Self
  where
    Self: IntoIterator<Item = A> + Sized + FromIterator<A>,
  {
    self.into_iter().skip_while(predicate).collect()
  }

  fn sorted(self) -> Self
  where
    A: Ord;

  fn sorted_by(self, compare: impl FnMut(&A, &A) -> Ordering) -> Self;

  fn sorted_unstable(self) -> Self
  where
    A: Ord;

  fn sorted_unstable_by(self, compare: impl FnMut(&A, &A) -> Ordering) -> Self;

  fn step_by(self, step: usize) -> Self
  where
    Self: IntoIterator<Item = A> + Sized + FromIterator<A>,
  {
    self.into_iter().step_by(step).collect()
  }

  fn tail(self) -> Self
  where
    Self: IntoIterator<Item = A> + Sized + FromIterator<A>,
  {
    let mut iterator = self.into_iter();
    iterator.next();
    iterator.collect()
  }

  fn take(self, n: usize) -> Self
  where
    Self: IntoIterator<Item = A> + Sized + FromIterator<A>,
  {
    self.into_iter().take(n).collect()
  }

  fn take_while(self, predicate: impl FnMut(&A) -> bool) -> Self
  where
    Self: IntoIterator<Item = A> + Sized + FromIterator<A>,
  {
    self.into_iter().take_while(predicate).collect()
  }

  fn unit(value: A) -> Self
  where
    Self: FromIterator<A>,
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

  fn zip<I>(self, iterable: I) -> Self::This<(A, I::Item)>
  where
    I: IntoIterator,
    Self: IntoIterator<Item = A> + Sized,
    Self::This<(A, I::Item)>: FromIterator<(A, I::Item)>,
  {
    self.into_iter().zip(iterable).collect()
  }
}
