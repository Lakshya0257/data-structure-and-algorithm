impl Solution {
    pub fn minimum_replacement(nums: Vec<i32>) -> i64 {
        let mut operations: i64 = 0;
        let mut prev_bound: i64 = *nums.last().unwrap() as i64;
        
        for &num in nums.iter().rev().skip(1) {
            let num: i64 = num as i64;
            let no_of_times: i64 = (num + prev_bound - 1) / prev_bound;
            operations += no_of_times - 1;
            prev_bound = num / no_of_times;
        }
        
        operations
    }
}