impl Solution {
    pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        let mut sorted_intervals = intervals;
        sorted_intervals.sort_by(|a, b| a[1].cmp(&b[1]));

        let mut count = 1;
        let mut ending = sorted_intervals[0][1];

        for i in 1..sorted_intervals.len() {
            if sorted_intervals[i][0] >= ending {
                count += 1;
                ending = sorted_intervals[i][1];
            }
        }

        sorted_intervals.len() as i32 - count
    }
}