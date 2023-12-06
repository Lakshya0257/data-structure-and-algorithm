impl Solution {
    pub fn total_money(n: i32) -> i32 {
        let mut j= 0;
        let mut k = 0;
        let mut arr = vec![];
        for _ in 0..n{
            if j < 7{           
                j += 1;
            }else{
                k +=1;
                j = 1;
            }
            arr.push(j+k);
        }
        arr.into_iter().sum()
    }
}