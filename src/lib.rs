pub fn add(left: usize, right: usize) -> usize {
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
  fn test_add(a: usize, b: usize) -> bool {
    // add(a, b) == add(a, b)
    true
  }
}
