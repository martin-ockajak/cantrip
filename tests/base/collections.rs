use std::collections::HashSet;
use std::hash::Hash;
use std::iter::{Product, Sum};
use crate::base::fixtures::{AggregableFixture, IterableFixture};
use crate::base::properties::*;

pub fn test_vec<Item>(data: Vec<Item>) -> bool
  where
    Item: Clone + Ord + IterableFixture,
{
  test_iterable(data.clone())
    && test_reverse_iterable(data.clone())
    && test_collectible(data.clone())
    && test_sequence(data.clone())
}

pub fn test_numeric_vec<Item>(data: Vec<Item>) -> bool
  where
    Item: Clone + Ord + IterableFixture + AggregableFixture + Sum + Product,
{
  test_vec(data.clone()) && test_numeric(data)
}

pub fn test_hash_set<Item>(data: HashSet<Item>) -> bool
  where
    Item: Clone + Ord + Eq + Hash + IterableFixture,
{
  test_iterable(data.clone()) && test_collectible(data.clone())
}

pub fn test_numeric_hash_set<Item>(data: HashSet<Item>) -> bool
  where
    Item: Clone + Ord + Eq + Hash + IterableFixture + AggregableFixture + Sum + Product,
{
  test_hash_set(data.clone()) && test_numeric(data)
}
