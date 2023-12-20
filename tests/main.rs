#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use std::collections::{HashMap, HashSet};

use crate::base::collections::*;

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

#[quickcheck]
fn hash_map_string(data: HashMap<String, String>) -> bool {
  true
}
