fn main() {
    println!("{:?}", find_words_containing(vec!["leet".to_string(),"code".to_string()], 'e'));
}
pub fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> {
    let mut result = vec![];
    for (index,word) in words.iter().enumerate() {
        'inner: for char in word.chars() {
            if char == x {
                result.push(index as i32);
                break 'inner;
            }
        }
    }
    result
}
