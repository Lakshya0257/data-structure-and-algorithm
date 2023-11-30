impl Solution {
  pub fn minimum_one_bit_operations(mut n: i32) -> i32 {
    let (mut res, mut curr) = (0, 1);
    while n != 0 {
      if n % 2 == 1 {
        res = curr - res;
      }
      curr = 2 * curr + 1;
      n /= 2;
    }
    return res;
  }
}