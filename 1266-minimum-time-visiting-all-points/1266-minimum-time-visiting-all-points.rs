impl Solution {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        let mut time = 0;
        let mut at = &points[0];
        for point in &points[1..] {
            time += (at[0] - point[0]).abs().max((at[1] - point[1]).abs());
            at = point;
        }
        time
    }
}