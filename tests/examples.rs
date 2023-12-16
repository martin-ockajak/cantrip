use cantrip::extensions::Sequence;

#[test]
fn overview() {
  use cantrip::extensions::*;

  let data = vec![0, 1, 2];

  data.map(|x| x + 1);                  // [1, 2, 3]: Vec<i32>

  data.fold(0, |r, x| r + x);           // 3: i32

  data.any(|&x| x == 0);                // true: bool

  data.clone().filter(|&x| x > 0);      // [1, 2]: Vec<i32>

  data.clone().add(0).distinct();       // [0, 1, 2]: Vec<i32>

  data.clone().delete(0).tail();        // [2]: Vec<i32>

  data.clone().group_by(|x| x % 2);     // {[0, 2], [1]}: HashMap<i32, Vec<i32>>

  data.clone().partition(|&x| x > 1);   // ([2], [0, 1]): (Vec<i32>, Vec<i32>)

  data.clone().zip(data);               // [(0, 0), (1, 1), (2, 2)]: Vec<(i32, i32)>
}
