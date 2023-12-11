impl Solution {
    pub fn find_special_integer(arr: Vec<i32>) -> i32 {
        [arr[arr.len()/4], arr[arr.len()/2], arr[arr.len()/4*3]]
            .iter()
            .fold(arr[arr.len()-1], |ans, &n| {
                let s = arr.partition_point(|&x| x < n);
                let e = arr.partition_point(|&x| x <= n);
                if (e-s) > arr.len()/4 {n} else {ans}
            })
    }
}