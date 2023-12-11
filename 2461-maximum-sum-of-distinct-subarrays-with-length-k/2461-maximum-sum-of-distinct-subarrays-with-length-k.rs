use::std::collections::HashMap;
impl Solution {
    pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let mut left = 0;
        let mut right = 0;
        let mut map = HashMap::new();
        let mut sum: i64 = 0;
        let mut curSum :i64 = 0;
        while right<nums.len() {
            if (right-left) > (k-1) as usize {
                map.remove(&nums[left]);
                curSum-=nums[left] as i64;
                left+=1;
            }
            if !map.contains_key(&nums[right]) {
                map.insert(nums[right],1);
                curSum+=nums[right] as i64;
            } else {
                map.clear();
                right=left+1;
                left=right;
                map.insert(nums[right],1);
                curSum = nums[right] as i64;
            }
            if(right-left) == (k-1) as usize {
                sum=sum.max(curSum);
            }
            right+=1;
        }
        sum
    }
}