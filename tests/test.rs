#[cfg(test)]
mod tests {
  use cantrip::extensions::*;

  #[test]
  fn test_x() {
    let data = vec![1, 2, 3];
    data.all(|x| x % 2 == 0);
    data.fold(0, |r, x| r + x);
    data.clone().zip(&data);
    assert_eq!(1, 1)
  }
}
