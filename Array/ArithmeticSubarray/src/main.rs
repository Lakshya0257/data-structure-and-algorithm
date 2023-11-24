fn main() {
    println!("{:?}",check_arithmetic_subarrays(vec![4,6,5,9,3,7], vec![0,0,2], vec![2,3,5]));
}

// pub fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
//     let mut result: Vec<bool> = vec![false; l.len()];
//     'outer: for i in 0..l.len() {
//         let mut copied_vector  = nums.get(l[i] as usize..=r[i] as usize).unwrap().to_vec();
//         copied_vector.sort_unstable();
//         let mut diff = 0;
//         for j in 1..copied_vector.len() {
//             if j==1 {
//                 diff = copied_vector[j]-copied_vector[0];
//             } else {
//                 if copied_vector[j]-copied_vector[j-1] != diff {
//                     continue 'outer;
//                 }
//             }
//         }
//         result[i] = true;
//     }
//     result
// }

pub fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
    let mut result = Vec::with_capacity(l.len());

    for (lo, hi) in l.into_iter().zip(r.into_iter()) {
        let mut sub_arr = nums[lo as usize..=hi as usize].to_vec();
        sub_arr.sort_unstable();

        let gap = sub_arr[1] - sub_arr[0];
        let is_arithmetic = sub_arr.iter()
            .skip(2)
            .zip(sub_arr.iter().skip(1))
            .all(|(a, b)| a - b == gap);

        result.push(is_arithmetic);
    }

    result
}
