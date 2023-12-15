#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use crate::extensions::*;
use std::collections::HashSet;

mod extensions;

impl Fixture for String {
  fn test(&self) -> bool {
    self.len() % 2 == 0
  }

  fn add(&self, value: &Self) -> Self {
    if self.len() > u16::MAX as usize {
      self.clone()
    } else {
      self.clone() + value
    }
  }
}

impl Fixture for i64 {
  fn test(&self) -> bool {
    self % 2 == 0
  }

  fn add(&self, value: &Self) -> Self {
    self.saturating_add(value.clone())
  }
}

impl NumericFixture for i64 {
  fn init_mul() -> Self {
    1
  }

  fn checked_add(&self, value: Self) -> Option<Self> {
    self.checked_add(value)
  }

  fn checked_mul(&self, value: Self) -> Option<Self> {
    self.checked_mul(value)
  }
}

#[quickcheck]
fn vec_string(data: Vec<String>) -> bool {
  test_iterable(data.clone()) && test_ordered(data.clone())
}

#[quickcheck]
fn vec_i64(data: Vec<i64>) -> bool {
  test_iterable(data.clone())
    && test_ordered(data.clone())
    && test_aggregable(data.clone())
}

#[quickcheck]
fn hash_set_string(data: HashSet<String>) -> bool {
  test_iterable(data.clone())
}

#[quickcheck]
fn hash_set_i64(data: HashSet<String>) -> bool {
  test_iterable(data.clone())
}
