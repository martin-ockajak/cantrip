use std::collections::{LinkedList, VecDeque};

pub trait Append<A> {
  fn append(&mut self, value: A);
}

impl<A> Append<A> for Vec<A> {
  fn append(&mut self, value: A) {
    self.push(value)
  }
}

impl<A> Append<A> for LinkedList<A> {
  fn append(&mut self, value: A) {
    self.push_back(value)
  }
}

impl<A> Append<A> for VecDeque<A> {
  fn append(&mut self, value: A) {
    self.push_back(value)
  }
}
