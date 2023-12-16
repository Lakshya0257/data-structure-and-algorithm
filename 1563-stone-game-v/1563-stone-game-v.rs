impl Solution {
    pub fn find_sum(stone_value: &[i32]) -> i32 {
        stone_value.iter().sum()
    }

    pub fn find_min(stone_value: &[i32], start: usize, end: usize, mut pre_sum: i32, sum: i32, dp: &mut Vec<Vec<i32>>) -> i32 {
        if start == end {
            return 0;
        }

        if dp[start][end] != -1 {
            return dp[start][end];
        }

        let mut ans = i32::MIN;

        for i in start..end {
            pre_sum += stone_value[i];

            if pre_sum < sum - pre_sum {
                ans = ans.max(pre_sum + Solution::find_min(stone_value, start, i, 0, pre_sum, dp));
            } else if pre_sum > sum - pre_sum {
                ans = ans.max(sum - pre_sum + Solution::find_min(stone_value, i + 1, end, 0, sum - pre_sum, dp));
            } else {
                let val = (pre_sum + Solution::find_min(stone_value, start, i, 0, pre_sum, dp)).max(sum - pre_sum + Solution::find_min(stone_value, i + 1, end, 0, sum - pre_sum, dp));
                ans = ans.max(val);
            }
        }

        dp[start][end] = ans;
        ans
    }

    pub fn stone_game_v(stone_value: Vec<i32>) -> i32 {
        let sum = Solution::find_sum(&stone_value);

        let mut dp: Vec<Vec<i32>> = vec![vec![-1; stone_value.len()]; stone_value.len()];

        Solution::find_min(&stone_value, 0, stone_value.len() - 1, 0, sum, &mut dp)
    }
}