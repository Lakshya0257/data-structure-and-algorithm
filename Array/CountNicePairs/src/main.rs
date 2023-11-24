use std::collections::HashMap;
use std::ops::Add;

fn main() {
    println!("{}",count_nice_pairs(vec![13,10,35,24,76]));
}

pub fn count_nice_pairs(nums: Vec<i32>) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut result = 0;
    for (i, num) in nums.iter().enumerate(){
        let sum = num - rev(*num);
        let mut count = map.entry(sum).or_insert(0);
        result = (result+*count)%1000000007;
        *count += 1;
    }
    result

}

pub fn rev(mut num: i32) -> i32 {
    let mut num_rev : i32 = 0;

    while num >= 10 {
        num_rev = num_rev*10 + num%10;
        num = (num / 10);
    }
    num_rev = num_rev*10 + num%10;
    num_rev
}
