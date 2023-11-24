fn main() {
    println!("Hello, world!");
}
pub fn add_minimum(word: String) -> i32 {
    let mut result = 0;
    let length = word.len()-1;
    for (index,c) in word.char_indices() {
        match index {
            0 => match c {
                'b' => result+=1,
                'c' => result+=2,
                _ => ()
            },
            _ => ()
        }
        match c {
            'a' => match index {
                i if i==length => result+=2,
                _ => match word.chars().nth(index+1).unwrap() {
                    'a' => result+=2,
                    'c' => result+=1,
                    _ => ()
                }
            },
            'b' => match index {
                i if i==length => result+=1,
                _ => match word.chars().nth(index+1).unwrap() {
                    'a' => result+=1,
                    'b' => result+=2,
                    _ => ()
                }
            },
            'c' => match index {
                i if i==length => (),
                _ => match word.chars().nth(index+1).unwrap() {
                    'b' => result+=1,
                    'c' => result+=2,
                    _ => ()
                }
            },
            _ => ()
        }
    }
    result
}