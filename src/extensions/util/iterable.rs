use std::collections::LinkedList;

pub trait Iterable {
  type Item<'collection>
  where
    Self: 'collection;

  type Iterator<'collection>: Iterator<Item = Self::Item<'collection>>
  where
    Self: 'collection;

  fn iterator<'c>(&'c self) -> Self::Iterator<'c>;
}

pub struct SliceIterator<'c, T> {
  pub iterator: core::slice::Iter<'c, T>,
}

impl<'c, T> Iterator for SliceIterator<'c, T> {
  type Item = &'c T;

  fn next(&mut self) -> Option<Self::Item> {
    self.iterator.next()
  }
}

impl<Item> Iterable for Vec<Item> {
  type Item<'c> = &'c Item
  where
      Item: 'c;

  type Iterator<'c> = SliceIterator<'c, Item>
  where
      Item: 'c;

  fn iterator<'c>(&'c self) -> Self::Iterator<'c> {
    SliceIterator { iterator: self.iter() }
  }
}

pub struct LinkedListIterator<'c, T> {
  pub iterator: std::collections::linked_list::Iter<'c, T>,
}

impl<'c, T> Iterator for LinkedListIterator<'c, T> {
  type Item = &'c T;

  fn next(&mut self) -> Option<Self::Item> {
    self.iterator.next()
  }
}

impl<Item> Iterable for LinkedList<Item> {
  type Item<'c> = &'c Item
  where
      Item: 'c;

  type Iterator<'c> = LinkedListIterator<'c, Item>
  where
      Item: 'c;

  fn iterator<'c>(&'c self) -> Self::Iterator<'c> {
    LinkedListIterator { iterator: self.iter() }
  }
}
