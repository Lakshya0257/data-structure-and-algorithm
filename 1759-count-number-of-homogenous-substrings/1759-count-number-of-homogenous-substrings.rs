impl Solution {
    pub fn count_homogenous(s: String) -> i32 {
        let mut count = s.len() as i32;
        let mut x = 0;
        let chars: Vec<char> = s.chars().collect();

        for i in 1..s.len() {
            if chars[i] == chars[i - 1] {
                x += 1;
                count = (count+x)%1000000007;
            } else {
                x = 0;
            }
        }
        count
    }
}
