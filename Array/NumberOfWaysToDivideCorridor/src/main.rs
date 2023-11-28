fn main() {
    println!("Hello, world!");
}

impl Solution {
    pub fn number_of_ways(corridor: String) -> i32 {
        let mut i = 0;
        let mut count = 0;
        let mut result = 1;
        let mut s_count = 0;
        let mut s_count_total = 0;
        let chars: Vec<char> = corridor.chars().collect();

        while i < corridor.len() {
            if chars[i] == 'S' {
                s_count_total += 1;

                if s_count == 2 {
                    s_count = 0;
                }
                s_count += 1;

                if count != 0 {
                    result = (result as i64 * (count + 1) as i64 % 1_000_000_007) as i32;
                    count = 0;
                }
            }

            if s_count == 2 && chars[i] == 'P' {
                count += 1;
            }

            i += 1;
        }

        if s_count_total < 2 || s_count == 1 {
            return 0;
        }

        result
    }
}
