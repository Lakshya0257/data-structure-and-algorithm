impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut m:Vec<Vec<i32>> = Vec::new();
        for (i,_) in matrix[0].iter().enumerate(){
            let mut n = Vec::new();
            for (j,_) in matrix.iter().enumerate(){
                n.insert(j, matrix[j][i]);
            }
            m.insert(i,n);
        }
        m
    }
}