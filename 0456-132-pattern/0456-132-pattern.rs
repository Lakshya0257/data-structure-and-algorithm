impl Solution {
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        let mut helper = Vec::new();
        let mut arr = vec![nums[0]];
        for i in 1..nums.len() {
            arr.push(arr[i-1].min(nums[i]));
        }

        for (i , num) in nums.iter().enumerate().rev() {
            if *num > arr[i] {
                while helper.len()!=0 && helper.last().unwrap() <= &arr[i] {
                    helper.pop();
                }
                if helper.len()!=0 && num>helper.last().unwrap() {
                    return true;
                }
                helper.push(*num);
            }
        }
        return false;
    }
}