use cantrip::extensions::*;

#[test]
fn test() {
  let data = vec![1, 2, 3];
  data.map(|x| x + 1);
  data.flat_map(|x| unit(x + 1));
  data.fold(0, |r, x| r + x);
  data.any(|x| x % 2 == 0);
  data.clone().zip(&data);
  assert_eq!(1, 1)
}
