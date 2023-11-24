fn main() {
    println!("{:?}",combination_sum2(vec![10,1,2,7,6,1,5], 8));
}

pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut cand = candidates;
    cand.sort();
    let mut result: Vec<Vec<i32>> = Vec::new();
    comb_sum(&cand,target,&mut Vec::new(),&mut result);
    result
}

pub fn comb_sum(candidates: &[i32], target:i32, temp: &mut Vec<i32>, result:&mut Vec<Vec<i32>>){

    if target==0 {
        result.push(temp.clone());
        return;
    }

    for i in 0..candidates.len() {
        if target-candidates[i] < 0  {
            return;
        }
        temp.push(candidates[i]);
        comb_sum(&candidates[i+1..], target-candidates[i], temp, result);
        temp.pop();
    }
}