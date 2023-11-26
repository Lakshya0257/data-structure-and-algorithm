fn main() {
    println!("{:?}", get_sum_absolute_differences(vec![2,3,5]));
}

pub fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32> {
    let len : usize = nums.len() ;
    let mut pref = vec![0; len];
    let mut suff = vec![0; len];
    let mut result: Vec<i32> = Vec::new();

    for ((front_index, front_value), (back_index, back_value)) in nums.iter().enumerate().zip(nums.iter().enumerate().rev()){
        match front_index {
            0 => pref[0] = *front_value,
            _ => pref[front_index] = pref[front_index-1] + *front_value
        }
        match back_index {
            i if i==len-1 => suff[len-1] = *back_value,
            _ => suff[back_index] = suff[back_index+1] + *back_value
        }
    }

    for (index,num) in nums.iter().enumerate() {
        let x: i32 = (len as i32-1)*num;
        let y: i32 = (pref[index].max(suff[index]) - pref[index].min(suff[index]));
        result.push(x.max(y) - x.min(y));
    }

    result
}
