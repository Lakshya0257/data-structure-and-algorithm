use std::collections::HashMap;

impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let mut map = HashMap::new();

        for c in chars.chars() {
            map.entry(c)
                .and_modify(|existing_value| *existing_value += 1)
                .or_insert(1);
        }

        let mut count = 0;

        'outer: for word in words.iter() {
            let mut word_map = HashMap::new();

            for c in word.chars() {
                word_map
                    .entry(c)
                    .and_modify(|existing_value| *existing_value += 1)
                    .or_insert(1);
            }

            for (key, value) in word_map.iter() {
                if !map.contains_key(key) || *map.get(key).unwrap() < *value {
                    continue 'outer;
                }
            }
            count += word.len() as i32;
        }
        count
    }
}
