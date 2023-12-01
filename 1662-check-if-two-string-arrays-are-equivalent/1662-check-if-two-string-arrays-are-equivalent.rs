impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        let mut s : String = String::new();
        let mut y: String = String::new();
        for x in word1{
            s.push_str(&x);
        }
        for x in word2{
            y.push_str(&x);
        }
        s==y
    }
}