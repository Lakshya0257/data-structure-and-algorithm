impl Solution {
    pub fn average_value(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut count = 0;
        for x in nums {
            if x & 1 == 0 && x%3 == 0 {
                sum+=x;
                count+=1;
            }
        }
        if sum==0 {
            return 0;
        }
        sum/count
    }
}