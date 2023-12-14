use cantrip::extensions::*;

#[test]
fn test() {
  let data = vec![1, 2, 3];
  let map: Vec<i32> = data.map(|x| x + 1);
  let flat_map: Vec<i32> = data.flat_map(|x| unit(x + 1));
  let fold: i32 = data.fold(0, |r, x| r + x);
  let any: bool = data.any(|x| x % 2 == 0);
  let zip: Vec<(i32, i32)> = data.clone().zip(&data);
  assert_eq!(1, 1)
}
