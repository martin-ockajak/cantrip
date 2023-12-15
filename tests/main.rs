#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use crate::extensions::*;
use std::cmp::Ordering;

mod extensions;

#[quickcheck]
fn vec_string(data: Vec<String>) -> bool {
  test_iterable(data.clone(), String::new(), test_string, add_string, compare_string)
    && test_ordered(data.clone(), test_string)
}

#[quickcheck]
fn vec_i64(data: Vec<i64>) -> bool {
  test_iterable(data.clone(), 0, test_i64, add_i64, compare_i64)
    && test_ordered(data.clone(), test_i64)
    && test_aggregable(data.clone(), 0, |x, y| x.checked_add(y), 1, |x, y| x.checked_mul(y))
}

fn test_string(value: &String) -> bool {
  value.len() % 2 == 0
}

fn test_i64(value: &i64) -> bool {
  value % 2 == 0
}

fn add_string(value1: String, value2: &String) -> String {
  if value1.len() > u16::MAX as usize {
    value1
  } else {
    value1 + value2
  }
}

fn add_i64(value1: i64, value2: &i64) -> i64 {
  value1.saturating_add(value2.clone())
}

fn compare_string(value1: &String, value2: &String) -> Ordering {
  value1.cmp(value2)
}

fn compare_i64(value1: &i64, value2: &i64) -> Ordering {
  value1.cmp(value2)
}
