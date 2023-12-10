impl Solution {
    pub fn get_good_indices(variables: Vec<Vec<i32>>, target: i32) -> Vec<i32> {
    let mut ans = Vec::new();
    let mut k = 0;

    for x in variables {
        let m = x[0];
        let b = x[1];
        let c = x[2];
        let d = x[3];

        let mut n1 = 1;
        for _ in 0..b {
            n1 = (n1 * m) % 10;
        }

        let mut n2 = 1;
        for _ in 0..c {
            n2 = (n2 * n1) % d;
        }

        if n2 == target {
            ans.push(k as i32);
        }

        k += 1;
    }

    ans
}
}