impl Solution {
    pub fn num_subarray_bounded_max(nums: Vec<i32>, left: i32, right: i32) -> i32 {
        (Self::count_subarrays(&nums, right) - Self::count_subarrays(&nums, left - 1)) as i32
    }

    fn count_subarrays(nums: &Vec<i32>, bound: i32) -> i64 {
        let mut count = 0;
        let mut val = 0;
        let mut max_val = i32::MIN;

        for &num in nums.iter() {
            max_val = max_val.max(num);

            if max_val <= bound {
                count += 1;
            } else {
                val += count * (count + 1) / 2;
                count = 0;
                max_val = i32::MIN;
            }
        }

        val += count * (count + 1) / 2;
        val
    }
}
