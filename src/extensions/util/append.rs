use std::collections::{LinkedList, VecDeque};

pub(crate) trait Append<Item> {
  fn append(&mut self, value: Item);
}

impl<Item> Append<Item> for Vec<Item> {
  #[inline]
  fn append(&mut self, value: Item)
  where
    Self: Extend<Item>,
  {
    self.push(value)
  }
}

impl<Item> Append<Item> for LinkedList<Item> {
  #[inline]
  fn append(&mut self, value: Item) {
    self.push_back(value)
  }
}

impl<Item> Append<Item> for VecDeque<Item> {
  #[inline]
  fn append(&mut self, value: Item) {
    self.push_back(value)
  }
}
