impl Solution {
    pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let (mut res, mut cur, mut dup, k) = (0i64, 0i64, -1i32, k as usize);
        let mut pos = vec![-1i32; 100001];

        for i in 0..nums.len() {
            cur += nums[i] as i64;
            if (i >= k) {cur -= nums[i-k] as i64;}
            dup = dup.max(pos[nums[i] as usize]);
            if ((i-k) as i32 >= dup) {res = res.max(cur);}
            pos[nums[i] as usize] = i as i32;
        }

        return res;
    }
}