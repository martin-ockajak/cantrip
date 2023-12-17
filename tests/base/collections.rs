use std::collections::HashSet;
use std::hash::Hash;
use std::iter::{Product, Sum};
use crate::base::fixtures::{AggregableFixture, TraversableFixture};
use crate::base::properties::*;

pub fn test_vec<Item>(data: Vec<Item>) -> bool
  where
    Item: Clone + Ord + TraversableFixture,
{
  test_traversable(data.clone())
    && test_ordered(data.clone())
    && test_collectible(data.clone())
    && test_sequence(data.clone())
}

pub fn test_numeric_vec<Item>(data: Vec<Item>) -> bool
  where
    Item: Clone + Ord + TraversableFixture + AggregableFixture + Sum + Product,
{
  test_vec(data.clone()) && test_aggregable(data)
}

pub fn test_hash_set<Item>(data: HashSet<Item>) -> bool
  where
    Item: Clone + Ord + Eq + Hash + TraversableFixture,
{
  test_traversable(data.clone()) && test_set(data.clone())
}

pub fn test_numeric_hash_set<Item>(data: HashSet<Item>) -> bool
  where
    Item: Clone + Ord + Eq + Hash + TraversableFixture + AggregableFixture + Sum + Product,
{
  test_hash_set(data.clone()) && test_aggregable(data)
}
