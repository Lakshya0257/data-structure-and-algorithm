use std::collections::HashMap;

fn main() {
    println!("{:?}",group_anagrams(vec!["".to_string(),"".to_string()]));
}

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map = HashMap::new();
    for str in strs.iter() {
        let mut chars: Vec<char> = str.clone().chars().collect();
        chars.sort();
        let key: String = chars.into_iter().collect();
        map.entry(key).and_modify(|lst : &mut Vec<String>|{lst.push(str.clone())}).or_insert(vec![str.clone()]);
    }
    let mut result = Vec::new();
    for (key,value) in map.iter() {
        result.push(value.to_owned());
    }
    result
}