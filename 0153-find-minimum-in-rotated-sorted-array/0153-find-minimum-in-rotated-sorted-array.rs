impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut result = 5001;
        let mut low = 0;
        let mut high = nums.len()-1;
        while low<=high {
            if nums[low]<=nums[high] {
                return result.min(nums[low]);
            }
            let index = (low+high)/2;
            if nums[low]<=nums[index]{
                low = index + 1;
                result = result.min(nums[low]);
            } else {
                high = index -1;
                result = result.min(nums[index]);
            }
        }
        result
    }
}