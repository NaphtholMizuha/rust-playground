use std::vec;

pub fn min(vals: &[usize]) -> usize {
    *vals.iter().min().unwrap()
}

pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
    let mut max_size: usize = 0;
    let mut dp = vec![vec![0 as usize; matrix[0].len()]; matrix.len()];
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if matrix[i][j] == '0' {
                dp[i][j] = 0;
            } else {
                let left = if i <= 0 {0} else {dp[i - 1][j]};
                let up = if j <= 0 {0} else {dp[i][j - 1]};
                let nw = if i <= 0 || j <= 0 {0} else {dp[i - 1][j - 1]};
                dp[i][j] = min(&[left, up, nw]) + 1
            }
            max_size = if dp[i][j] > max_size {dp[i][j]} else {max_size}
        }
    }
    let max_size = max_size as i32;
    max_size * max_size
}
fn main() {
    let matrix = vec![
        vec!['1', '0', '1', '0', '0'],
        vec!['1', '0', '1', '1', '1'],
        vec!['1', '1', '1', '1', '1'],
        vec!['1', '0', '0', '1', '0'],
    ];

    println!("{}", maximal_square(matrix))
}