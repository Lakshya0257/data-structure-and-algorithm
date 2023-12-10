use std::collections::HashMap;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut map = HashMap::new();
        let mut sum = 0;
        let mut result = 0;
        for &num in nums.iter() {
            sum += num;
            if sum == k {
                result += 1;
            }
            if let Some(&count) = map.get(&(sum - k)) {
                result += count;
            }
            *map.entry(sum).or_insert(0) += 1;
        }

        result
    }
}
