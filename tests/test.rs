use std::iter;
use cantrip::extensions::*;

#[test]
fn test() {
  let data = vec![0, 1, 2];

  data.map(|x| x + 1);                     // Vec<i32>

  data.filter(|x| x > 0);                  // Vec<i32>

  data.flat_map(|x| iter::once(x + 1));    // Vec<i32>

  data.fold(0, |r, x| r + x);              // i32

  data.any(|x| x == 0);                    // bool

  data.clone().zip(&data);                 // Vec<(i32, i32)>
  assert_eq!(1, 1)
}
