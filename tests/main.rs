#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use std::collections::{HashMap, VecDeque};

use crate::extensions::collections::*;

mod extensions;

#[test]
fn vectors() {
  test_vector::<Vec<i64>>();
  test_vector::<VecDeque<i64>>();
}

#[quickcheck]
fn hash_map_string(data: HashMap<String, String>) -> bool {
  data.len();
  let x = "".to_string();
  true
}
