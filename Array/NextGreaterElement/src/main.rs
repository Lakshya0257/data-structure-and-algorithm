fn main() {
    println!("{:?}",next_greater_element( vec![4,1,2],vec![1,3,4,2]));
}

use std::collections::HashMap;

    pub fn next_greater_element(mut nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut map = HashMap::new();
        let mut lst = Vec::new();
        let x = nums2.len()-1;
        for i in (0..=x).rev() {
            while lst.len()!=0 && nums2[i]>lst[lst.len()-1] {
                lst.pop();
            }
            if lst.len()!=0 && nums2[i]<lst[lst.len()-1] {
                map.insert(nums2[i],lst[lst.len()-1]);
            }
            if lst.len()==0 {
                map.insert(nums2[i],-1);
            }
            lst.push(nums2[i]);
        }

        for i in 0..nums1.len(){
            nums1[i] = *map.get(&nums1[i]).unwrap();
        }

        nums1
    }
