fn main() {
    println!("Hello, world!");
}

    pub fn longest_alternating_subarray(nums: Vec<i32>, threshold: i32) -> i32 {
        let (mut curr, mut result) = (0, 0);
        for e in nums.into_iter() {
            if e > threshold {
                curr = 0;
                continue;
            }

            if e % 2 == curr % 2 {
                curr += 1;
            } else {
                curr = i32::from(e % 2 == 0);
            }
            result = result.max(curr);
        }
        result
    }

