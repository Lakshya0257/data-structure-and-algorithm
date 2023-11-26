use std::collections::HashMap;

fn main() {
    println!("{}",maximize_square_hole_area(3,2,vec![3,2,4],vec![3,2]));
}

pub fn maximize_square_hole_area(n: i32, m: i32, mut h_bars: Vec<i32>, mut v_bars: Vec<i32>) -> i32 {
    h_bars.sort_unstable();
    v_bars.sort_unstable();
    let mut map = HashMap::new();
    map.insert(1,1);
    let mut result = 4;
    let mut count = 1;
    for (index,num) in h_bars.iter().enumerate(){
        match index {
            0 => (),
            _ => {
                if *num == h_bars[index-1]+1 {
                    count+=1;
                    if !map.contains_key(&count) {
                        map.insert(count,1);
                    }
                }else {
                    if count!=0 && !map.contains_key(&count) {
                        map.insert(count,1);
                    }
                    count = 1;
                }
                if index==h_bars.len()-1 && count!=1 && !map.contains_key(&count) {
                    map.insert(count,1);
                }

            }
        }
    }
    let mut v_count = 1;
    for (index,num) in v_bars.iter().enumerate(){
        match index {
            0 => (),
            _ => {
                if *num == v_bars[index-1]+1 {
                    v_count+=1;
                    if map.contains_key(&v_count) {
                        result = result.max((v_count+1)*(v_count+1));
                    }
                }else {
                    if v_count!=0 && map.contains_key(&v_count) {
                        result = result.max((v_count+1)*(v_count+1));
                    }
                    v_count = 1;
                }
            }
        }
    }

    result
}
