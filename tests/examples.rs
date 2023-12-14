#[test]
fn overview() {
  use cantrip::extensions::*;

  let data = vec![0, 1, 2];

  data.map(|x| x + 1);                     // Vec<i32>

  data.filter(|x| x > 0);                  // Vec<i32>

  data.fold(0, |r, x| r + x);              // i32

  data.any(|x| x == 0);                    // bool

  data.head();                             // Option<i32>

  data.clone().add(3);                     // Vec<i32>

  data.clone().zip(data);                  // Vec<(i32, i32)>
}
