#[cfg(test)]
mod tests {
  use crate::extensions::traits::*;

  #[test]
  fn test_x() {
    let data = vec![1, 2, 3];
    let x = data.all(|x| x % 2 == 0);
    let y = data.zip(&data);
    let z = data.fold(0, |x| x + 1);
    assert_eq!(1, 1)
  }
}
