#![allow(missing_docs)]

use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};

pub trait Iterable {
  type Item<'collection>
  where
    Self: 'collection;

  type Iterator<'collection>: Iterator<Item = Self::Item<'collection>>
  where
    Self: 'collection;

  #[allow(clippy::needless_lifetimes)]
  fn iterator<'c>(&'c self) -> Self::Iterator<'c>;
}

#[derive(Debug)]
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

  #[allow(clippy::needless_lifetimes)]
  fn iterator<'c>(&'c self) -> Self::Iterator<'c> {
    SliceIterator { iterator: self.iter() }
  }
}

#[derive(Debug)]
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

  #[allow(clippy::needless_lifetimes)]
  fn iterator<'c>(&'c self) -> Self::Iterator<'c> {
    LinkedListIterator { iterator: self.iter() }
  }
}

#[derive(Debug)]
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

  #[allow(clippy::needless_lifetimes)]
  fn iterator<'c>(&'c self) -> Self::Iterator<'c> {
    VecDequeIterator { iterator: self.iter() }
  }
}

#[derive(Debug)]
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

  #[allow(clippy::needless_lifetimes)]
  fn iterator<'c>(&'c self) -> Self::Iterator<'c> {
    HashSetIterator { iterator: self.iter() }
  }
}

#[derive(Debug)]
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

  #[allow(clippy::needless_lifetimes)]
  fn iterator<'c>(&'c self) -> Self::Iterator<'c> {
    BTreeSetIterator { iterator: self.iter() }
  }
}

#[derive(Debug)]
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

  #[allow(clippy::needless_lifetimes)]
  fn iterator<'c>(&'c self) -> Self::Iterator<'c> {
    BinaryHeapIterator { iterator: self.iter() }
  }
}

#[derive(Debug)]
pub struct HashMapIterator<'c, Key, Value> {
  pub iterator: std::collections::hash_map::Iter<'c, Key, Value>,
}

impl<'c, Key, Value> Iterator for HashMapIterator<'c, Key, Value> {
  type Item = (&'c Key, &'c Value);

  fn next(&mut self) -> Option<Self::Item> {
    self.iterator.next()
  }
}

impl<Key, Value> Iterable for HashMap<Key, Value> {
  type Item<'c> = (&'c Key, &'c Value)
    where
      Key: 'c,
      Value: 'c;

  type Iterator<'c> = HashMapIterator<'c, Key, Value>
    where
      Key: 'c,
      Value: 'c;

  #[allow(clippy::needless_lifetimes)]
  fn iterator<'c>(&'c self) -> Self::Iterator<'c> {
    HashMapIterator { iterator: self.iter() }
  }
}

#[derive(Debug)]
pub struct BTreeMapIterator<'c, Key, Value> {
  pub iterator: std::collections::btree_map::Iter<'c, Key, Value>,
}

impl<'c, Key, Value> Iterator for BTreeMapIterator<'c, Key, Value> {
  type Item = (&'c Key, &'c Value);

  fn next(&mut self) -> Option<Self::Item> {
    self.iterator.next()
  }
}

impl<Key, Value> Iterable for BTreeMap<Key, Value> {
  type Item<'c> = (&'c Key, &'c Value)
    where
      Key: 'c,
      Value: 'c;

  type Iterator<'c> = BTreeMapIterator<'c, Key, Value>
    where
      Key: 'c,
      Value: 'c;

  #[allow(clippy::needless_lifetimes)]
  fn iterator<'c>(&'c self) -> Self::Iterator<'c> {
    BTreeMapIterator { iterator: self.iter() }
  }
}
