fn main() {
    println!("{}",find_length_of_shortest_subarray(vec![1,2,2,2,2,2,3,1,7,5,1,2,2,2,2,2,2,5,6]));
}

pub fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32 {
    let mut pref_arr : Vec<i32> = Vec::new();
    let mut suffix_arr : Vec<i32> = Vec::new();
    let mut result: i32 = 0;

    for num in arr.iter() {
        match pref_arr.len() {
            0 => pref_arr.push(*num),
            _ => match *num>=pref_arr[pref_arr.len()-1] {
                true => pref_arr.push(*num),
                _ => break
            }
        }
    }
    for num in arr.iter().rev() {
        match suffix_arr.len() {
            0 => suffix_arr.push(*num),
            _ => match *num<=suffix_arr[suffix_arr.len()-1] {
                true => suffix_arr.push(*num),
                _ => break
            }
        }
    }

    if pref_arr.len()==arr.len() {
        return result;
    }

    result = (arr.len() - (pref_arr.len() + suffix_arr.len())) as i32;



    while pref_arr[pref_arr.len()-1]>suffix_arr[suffix_arr.len()-1] {
        let pre_jumps = find_index(&pref_arr, suffix_arr[suffix_arr.len()-1], "prefix");
        let suf_jumps = find_index(&suffix_arr, pref_arr[pref_arr.len()-1], "suffix");

        if pre_jumps >= suf_jumps {
            suffix_arr.pop();
        }else {
            pref_arr.pop();
        }
        result+=1;
        if pref_arr.len()==0 || suffix_arr.len()==0 {
            return result;
        }
    }


    result
}

pub fn find_index(arr: &Vec<i32>, num: i32, tpe: &str) -> i32 {
    match tpe {
        "prefix" => if arr[0] > num {
            return arr.len() as i32;
        },
        _ => if arr[0] < num {
            return arr.len() as i32;
        }
    }

    let mut left = 0;
    let mut right = arr.len()-1;

    while left+1<right {
        let index = (left+right)/2;
        if arr[index]<num {
            match tpe {
                "prefix" => left=index,
                _ => right = index
            }
        } else if arr[index]==num {
            left=index;
        } else {
            match tpe {
                "suffix" => left=index,
                _ => right = index
            }
        }
    }

    (arr.len() - left -1) as i32
}