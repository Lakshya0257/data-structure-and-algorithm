fn main() {
    println!("{}",max_moves(vec![vec![2,4,3,5],vec![5,4,9,3],vec![3,4,2,11],vec![10,9,13,15]]));
}

pub fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
    let mut moves = 0;
    for i in 0..grid.len() {
        moves=moves.max(count_moves(&grid,0,i,0,&mut vec![vec![false; grid[0].len()]; grid.len()]))
    }
    moves

}

pub fn count_moves(grid: &Vec<Vec<i32>>, column: usize, row: usize, moves: i32, visited: &mut Vec<Vec<bool>>) -> i32 {
    if visited[row][column]==true {
        return 0;
    };
    if column==grid[0].len()-1 {
        return moves;
    };
    visited[row][column]=true;
    let mut mov: i32 = moves;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut z: i32 = 0;
    if row!=0 && grid[row][column] < grid[row-1][column+1]{
        x = x.max(count_moves(grid, column + 1, row - 1, mov + 1,visited));
    }
    if row!=grid.len()-1 && grid[row][column] < grid[row+1][column+1] {
        y = y.max(count_moves(grid, column + 1, row + 1, mov + 1,visited));
    }
    if grid[row][column] < grid[row][column+1] {
        z = z.max(count_moves(grid, column+1, row, mov+1,visited));
    }
    mov = mov.max(x.max(y.max(z)));
    mov
}
