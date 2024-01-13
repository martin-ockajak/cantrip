use std::collections::{BTreeSet, BinaryHeap, HashSet, LinkedList, VecDeque};

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

pub struct VecDequeIterator<'c, T> {
  pub iterator: std::collections::vec_deque::Iter<'c, T>,
}

impl<'c, T> Iterator for VecDequeIterator<'c, T> {
  type Item = &'c T;

  fn next(&mut self) -> Option<Self::Item> {
    self.iterator.next()
  }
}

impl<Item> Iterable for VecDeque<Item> {
  type Item<'c> = &'c Item
    where
      Item: 'c;

  type Iterator<'c> = VecDequeIterator<'c, Item>
    where
      Item: 'c;

  fn iterator<'c>(&'c self) -> Self::Iterator<'c> {
    VecDequeIterator { iterator: self.iter() }
  }
}

pub struct HashSetIterator<'c, T> {
  pub iterator: std::collections::hash_set::Iter<'c, T>,
}

impl<'c, T> Iterator for HashSetIterator<'c, T> {
  type Item = &'c T;

  fn next(&mut self) -> Option<Self::Item> {
    self.iterator.next()
  }
}

impl<Item> Iterable for HashSet<Item> {
  type Item<'c> = &'c Item
    where
      Item: 'c;

  type Iterator<'c> = HashSetIterator<'c, Item>
    where
      Item: 'c;

  fn iterator<'c>(&'c self) -> Self::Iterator<'c> {
    HashSetIterator { iterator: self.iter() }
  }
}

pub struct BTreeSetIterator<'c, T> {
  pub iterator: std::collections::btree_set::Iter<'c, T>,
}

impl<'c, T> Iterator for BTreeSetIterator<'c, T> {
  type Item = &'c T;

  fn next(&mut self) -> Option<Self::Item> {
    self.iterator.next()
  }
}

impl<Item> Iterable for BTreeSet<Item> {
  type Item<'c> = &'c Item
    where
      Item: 'c;

  type Iterator<'c> = BTreeSetIterator<'c, Item>
    where
      Item: 'c;

  fn iterator<'c>(&'c self) -> Self::Iterator<'c> {
    BTreeSetIterator { iterator: self.iter() }
  }
}

pub struct BinaryHeapIterator<'c, T> {
  pub iterator: std::collections::binary_heap::Iter<'c, T>,
}

impl<'c, T> Iterator for BinaryHeapIterator<'c, T> {
  type Item = &'c T;

  fn next(&mut self) -> Option<Self::Item> {
    self.iterator.next()
  }
}

impl<Item> Iterable for BinaryHeap<Item> {
  type Item<'c> = &'c Item
    where
      Item: 'c;

  type Iterator<'c> = BinaryHeapIterator<'c, Item>
    where
      Item: 'c;

  fn iterator<'c>(&'c self) -> Self::Iterator<'c> {
    BinaryHeapIterator { iterator: self.iter() }
  }
}
