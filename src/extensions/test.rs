#[cfg(test)]
mod tests {
  use crate::extensions::*;

  #[test]
  fn test_x() {
    let data = vec![1, 2, 3];
    data.all(|x| x % 2 == 0);
    data.fold(0, |r, x| r + x);
    data.zip(&data);
    assert_eq!(1, 1)
  }
}
