impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let mut result = String::new();
        for i in 0..nums.len() {
            if nums[i].chars().nth(i).unwrap() == '1' {
                result.push('0');
            }else{
                result.push('1');
            }
        }
        result
    }
}