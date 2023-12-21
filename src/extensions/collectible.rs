use std::collections::HashSet;
use std::hash::Hash;
use std::iter::{Product, Sum};

pub trait Collectible<Item>: IntoIterator<Item = Item> + Sized  {
  type This<I>;

  fn diff(self, iterable: impl IntoIterator<Item = Item>) -> Self
  where
    Item: Eq + Hash,
    Self: FromIterator<Item>,
  {
    let mut removed: HashSet<Item> = HashSet::new();
    removed.extend(iterable);
    self.into_iter().filter(|x| !removed.contains(x)).collect()
  }

  fn intersect(self, iterable: impl IntoIterator<Item = Item>) -> Self
  where
    Item: Eq + Hash,
    Self: FromIterator<Item>,
  {
    let mut retained: HashSet<Item> = HashSet::new();
    retained.extend(iterable);
    self.into_iter().filter(|x| retained.contains(x)).collect()
  }

  fn reduce(self, function: impl FnMut(Item, Item) -> Item) -> Option<Item>
  {
    let mut iterator = self.into_iter();
    iterator.next().map(|result| iterator.fold(result, function))
  }

  // fn largest_by(self, n: usize, compare: impl FnMut(&Item, &Item) -> Ordering) -> Self
  // where
  //   Item: Ord,
  //   Self: IntoIterator<Item = Item> + Sized + Default + Extend<Item>,
  // {
  //   largest_by(self, n, compare)
  // }

  fn product(self) -> Item
    where
      Item: Product,
  {
    self.into_iter().product()
  }

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
