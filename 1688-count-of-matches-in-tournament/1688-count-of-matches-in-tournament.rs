impl Solution {
    pub fn number_of_matches(mut n: i32) -> i32 {
        let mut matches = 0;
        while n>1 {
            match n%2 {
                0 => {
                    matches+=n/2;
                    n/=2;
                },
                1 => {
                    matches+=(n-1)/2;
                    n=(n-1)/2 + 1;
                },
                _ => ()
            }
        }
        matches
    }
}