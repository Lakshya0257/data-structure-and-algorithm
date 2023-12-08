impl Solution {
    pub fn find_pairs(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let mut count = 0;
        let mut cur = i32::MIN;
        for (i,num) in nums.iter().enumerate() {
            if *num==cur {
                continue;
            }
            cur = *num;
            let val = *num + k;
            if val>=*num && i!=nums.len()-1 && Self::binary_search(&nums,val,i+1) {
                count+=1;
            }
        }
        count
    }

    pub fn binary_search(nums: &[i32], k: i32, mut left: usize) -> bool {
    let mut right = nums.len() - 1;
    
    while left <= right {
        let index = (left + right) / 2;

        match nums[index] {
            i if i == k => return true,
            i if i < k => {
                left = index + 1;
            }
            _ => {
                right = index - 1;
            }
        }
    }

    false
}

}