#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use std::collections::HashSet;
use std::hash::Hash;
use std::iter::{Product, Sum};

use crate::base::fixtures::*;
use crate::base::properties::*;

mod base;

#[quickcheck]
fn vec_string(data: Vec<String>) -> bool {
  test_vec(data)
}

#[quickcheck]
fn vec_i64(data: Vec<i64>) -> bool {
  test_numeric_vec(data)
}

#[quickcheck]
fn hash_set_string(data: HashSet<String>) -> bool {
  test_hash_set(data)
}

#[quickcheck]
fn hash_set_i64(data: HashSet<i64>) -> bool {
  test_numeric_hash_set(data)
}

fn test_vec<Item>(data: Vec<Item>) -> bool
where
  Item: Clone + Ord + TraversableFixture,
{
  test_traversable(data.clone())
    && test_ordered(data.clone())
    && test_collectible(data.clone())
    && test_sequence(data.clone())
}

fn test_numeric_vec<Item>(data: Vec<Item>) -> bool
where
  Item: Clone + Ord + TraversableFixture + AggregableFixture + Sum + Product,
{
  test_vec(data.clone()) && test_aggregable(data)
}

fn test_hash_set<Item>(data: HashSet<Item>) -> bool
where
  Item: Clone + Ord + Eq + Hash + TraversableFixture,
{
  test_traversable(data.clone()) && test_set(data.clone())
}

fn test_numeric_hash_set<Item>(data: HashSet<Item>) -> bool
where
  Item: Clone + Ord + Eq + Hash + TraversableFixture + AggregableFixture + Sum + Product,
{
  test_hash_set(data.clone()) && test_aggregable(data)
}
