#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use crate::extensions::*;

mod extensions;

#[quickcheck]
fn vec_string(data: Vec<String>) -> bool {
  test_iterable(data.clone(), String::new(), test_string, fold_string) && test_ordered(data.clone(), test_string)
}

fn vec_i64(data: Vec<i64>) -> bool {
  test_iterable(data.clone(), 0, test_i64, fold_i64)
    && test_ordered(data.clone(), test_i64)
    && test_aggregable(data.clone(), 0, |x, y| x.checked_add(y), 1, |x, y| x.checked_mul(y))
}

fn test_string(value: &String) -> bool {
  value.len() % 2 == 0
}

fn test_i64(value: &i64) -> bool {
  value % 2 == 0
}

fn fold_string(result: String, value: &String) -> String {
  if result.len() > u16::MAX as usize {
    result
  } else {
    result + value
  }
}

fn fold_i64(result: i64, value: &i64) -> i64 {
  result.saturating_add(value.clone())
}

