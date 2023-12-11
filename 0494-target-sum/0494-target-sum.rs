impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let len_ns: usize = nums.len();
        let sum: i32 = nums.iter().sum();
        if target > sum || target < -sum{
            return 0;
        }
        let mut memo: Vec<Vec<Option<i32>>> = vec![vec![None; sum as usize * 2 + 1]; len_ns + 1];
        return Self::helper(0, sum, target + sum, &nums, &mut memo);
    }
    fn helper(idx: usize, sum: i32, target: i32, nums: &Vec<i32>, memo: &mut Vec<Vec<Option<i32>>>) -> i32{
        let len_ns: usize = nums.len();
        if idx == len_ns{
            return if sum == target { 1 } else { 0 };
        }
        if let Some(m) = memo[idx][sum as usize]{
            return m;
        }
        let res = Self::helper(idx + 1, sum + nums[idx], target, nums, memo) + Self::helper(idx + 1, sum - nums[idx], target, nums, memo);
        memo[idx][sum as usize] = Some(res);
        return res;
    }
}