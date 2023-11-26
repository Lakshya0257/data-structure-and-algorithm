use std::collections::HashMap;

fn main() {
    println!("{}",largest_rectangle_area(vec![0,0]));
}

pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    let mut lst = Vec::new();
    let len = heights.len();
    let mut left_map = HashMap::new();
    for i in 0..len {
        while lst.len()!=0 && heights[i]<=heights[*lst.last().unwrap() as usize] {
            lst.pop();
        }
        match lst.last() {
            Some(value) if heights[*value as usize] < heights[i] => {
                left_map.insert(i, *value);
            }
            None => {
                left_map.insert(i, -1);
            }
            _ => (),
        }
        lst.push(i as i32);
    }
    lst.clear();
    let mut right_map = HashMap::new();
    for i in (0..len).rev() {
        while lst.len()!=0 && heights[i]<=heights[*lst.last().unwrap() as usize] {
            lst.pop();
        }
        match lst.last() {
            Some(value) if heights[*value as usize] < heights[i] => {
                right_map.insert(i, *value);
            }
            None => {
                right_map.insert(i, len as i32);
            }
            _ => (),
        }
        lst.push(i as i32);
    }
    println!("{:?}",left_map);
    println!("{:?}",right_map);
    let mut height = 0;
    for i in 0..len {
        height=height.max(heights[i]);
        height=height.max(((*right_map.get(&i).unwrap()) - (*left_map.get(&i).unwrap() + 1))*heights[i]);
    }
    height
}
