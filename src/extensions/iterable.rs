pub trait Iterable {
  type Item<'collection>
  where
    Self: 'collection;

  type Iterator<'collection>: Iterator<Item = Self::Item<'collection>>
  where
    Self: 'collection;

  fn iterator<'c>(&'c self) -> Self::Iterator<'c>;
}

pub struct Iter<'c, T> {
  pub collection: &'c [T],
}

impl<'c, T> Iterator for Iter<'c, T> {
  type Item = &'c T;

  fn next(&mut self) -> Option<Self::Item> {
    if let Some((prefix_elem, suffix)) = self.collection.split_first() {
      self.collection = suffix;
      Some(prefix_elem)
    } else {
      None
    }
  }
}
