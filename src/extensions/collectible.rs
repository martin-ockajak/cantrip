use std::iter::{Product, Sum};

pub trait Collectible<Item>: IntoIterator<Item = Item> + Sized  {
  type This<I>;

  // fn largest_by(self, n: usize, compare: impl FnMut(&Item, &Item) -> Ordering) -> Self
  // where
  //   Item: Ord,
  //   Self: IntoIterator<Item = Item> + Sized + Default + Extend<Item>,
  // {
  //   largest_by(self, n, compare)
  // }

  #[inline]
  fn product(self) -> Item
    where
      Item: Product,
  {
    self.into_iter().product()
  }

  #[inline]
  fn sum(self) -> Item
    where
      Item: Sum,
  {
    self.into_iter().sum()
  }
}

// fn largest_by<Item, Collection>(
//   collection: Collection, n: usize, compare: impl FnMut(&Item, &Item) -> Ordering,
// ) -> Collection
// where
//   Item: Ord,
//   Collection: IntoIterator<Item = Item> + Sized + Default + Extend<Item>,
// {
//   let heap = BinaryHeap::new_by(compare);
//   heap.extend(collection);
//   let mut result = Collection::default();
//   for _ in 0..n {
//     result.extend(iter::once(heap.pop().unwrap()));
//   }
//   result
// }
