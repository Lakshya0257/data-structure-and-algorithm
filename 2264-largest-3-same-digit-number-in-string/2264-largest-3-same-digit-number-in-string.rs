impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        let mut target = -1;
        let num_chars: Vec<char> = num.chars().collect();
        for i in 0..num_chars.len() - 2 {
            if num_chars[i] == num_chars[i + 1] && num_chars[i] == num_chars[i + 2] {
                let current: i32 = num_chars[i..i + 3].iter().collect::<String>().parse().unwrap();
                if current > target {
                    target = current;
                }
            }
        }
        if target != -1 {
            if target == 0 {
                return format!("{}00", target);
            }
            return target.to_string();
        }
        "".to_string()
    }
}
