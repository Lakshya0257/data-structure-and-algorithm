fn main() {
    println!("{}",largest_submatrix(vec![
        vec![0, 0, 1],
        vec![1, 1, 1],
        vec![1, 0, 1]
    ]));
}

pub fn largest_submatrix(mut matrix: Vec<Vec<i32>>) -> i32 {
    let mut length = 0;
    matrix.sort_unstable();
    println!("{:?}",matrix);
    length
}
