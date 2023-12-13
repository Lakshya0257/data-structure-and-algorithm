impl Solution {
    pub fn minimum_deletions(s: String) -> i32 {
        let mut result = 0;
        let mut stack :Vec<char>= Vec::new();
        let chars : Vec<char> = s.chars().collect();
        for &c in chars.iter() {
            if c == 'a' {
                if stack.len()!=0 {
                    if stack.last().unwrap() == &'b' {
                        result+=1;
                    }
                    stack.pop();
                }
            }else {
                stack.push('b');
            }
        }
        result
    }
}