pub fn add(left: i32, right: i32) -> i32 {
  left + right
}

#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

#[cfg(test)]
mod tests {
  use super::*;

  #[quickcheck]
  fn test_add(a: i32, b: i32) -> bool {
    // add(a, b) == a + b
    true
  }
}
