impl Solution {
    pub fn average_value(nums: Vec<i32>) -> i32 {


        let mut count: i32 = 0;
        let mut total: i32 = 0;

        for n in nums.iter(){
            if n % 3 == 0 && n % 2 == 0 {
                count +=1;
                total +=n;
            }
        }
        
        if count > 0 {
            return total / count;
        }
        
        0
        
    }
}