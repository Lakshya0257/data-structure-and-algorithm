use std::collections::HashMap;

impl Solution {
    pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
        let mut map = HashMap::new();

        for val in dictionary.iter() {
            map.insert(val.clone(), 1);
        }

        let words: Vec<&str> = sentence.split_whitespace().collect();
        let mut result = String::new();

        for word in words.iter() {
            let mut wor = String::new();
            let chars: Vec<char> = word.chars().collect();

            for &c in chars.iter() {
                wor.push(c);
                if map.contains_key(&wor) {
                    break;
                }
            }

            result.push_str(&wor);
            result.push(' ');  
        }

        result.trim().to_string()  
    }
}
