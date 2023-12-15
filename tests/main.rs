#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use crate::extensions::*;

mod extensions;

#[quickcheck]
fn vec_i32(data: Vec<i32>) -> bool {
  test_iterable(data.clone(), predicate_i32)
    && test_ordered(data.clone(), predicate_i32)
    && test_aggregable(data.clone(), 0, |x, y| x.checked_add(y), 1, |x, y| x.checked_mul(y))
}

fn vec_str(data: Vec<String>) -> bool {
  test_iterable(data.clone(), predicate_string) && test_ordered(data.clone(), predicate_string)
}

fn predicate_string(value: &String) -> bool {
  value.len() % 2 == 0
}

fn predicate_i32(value: &i32) -> bool {
  value % 2 == 0
}
