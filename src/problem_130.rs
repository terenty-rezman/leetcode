struct Solution;

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        if board.is_empty() {
            return;
        }

        let m = board.len();
        let n = board[0].len();

        for i in 0..m {
            for j in 0..n {
                if j == 0 || j == n - 1 || i == 0 || i == m - 1 {
                    Self::dfs(board, i as i32, j as i32, m as i32, n as i32);
                }
            }
        }

        for i in 0..m {
            for j in 0..n {
                if board[i][j] == 'O' {
                    board[i][j] = 'X';
                } else if board[i][j] == '#' {
                    board[i][j] = 'O';
                }
            }
        }
    }

    fn dfs(board: &mut Vec<Vec<char>>, i: i32, j: i32, m: i32, n: i32) {
        if i < 0 || i >= m {
            return;
        }

        if j < 0 || j >= n {
            return;
        }

        if board[i as usize][j as usize] == 'O' {
            board[i as usize][j as usize] = '#';

            Self::dfs(board, i - 1, j, m, n);
            Self::dfs(board, i + 1, j, m, n);
            Self::dfs(board, i, j - 1, m, n);
            Self::dfs(board, i, j + 1, m, n);
        }
    }
}

pub mod tests {
    use super::Solution;

    pub fn test() {
        let mut input = [
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'O', 'X'],
            vec!['X', 'X', 'O', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ]
        .to_vec();
        Solution::solve(&mut input);
        dbg!(input);
    }
}
