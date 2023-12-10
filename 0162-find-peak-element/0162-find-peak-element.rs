impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let mut l = 0;
        let n = nums.len();
        let mut r = n - 1;
        let mut mid = 0;

        while l <= r {
            mid = (l + r) / 2;

            if (mid == 0 || nums[mid - 1] <= nums[mid]) && (mid == n - 1 || nums[mid + 1] <= nums[mid]) {
                break;
            }

            if mid > 0 && nums[mid - 1] > nums[mid] {
                r = mid - 1;
            } else {
                l = mid + 1;
            }
        }

        mid as i32
    }
}
