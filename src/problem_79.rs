struct Solution;

impl Solution {
    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if Self::dfs(
                    &mut board,
                    &word.chars().collect::<Vec<_>>(),
                    i as i32,
                    j as i32,
                ) {
                    return true;
                }
            }
        }

        false
    }

    fn dfs(board: &mut [Vec<char>], word: &[char], i: i32, j: i32) -> bool {
        if word.is_empty() {
            return true;
        }

        if i < 0 || i >= board.len() as i32 {
            return false;
        }

        if j < 0 || j >= board[0].len() as i32 {
            return false;
        }

        if word[0] != board[i as usize][j as usize] {
            return false;
        }

        // mark as used
        let old_c = board[i as usize][j as usize];
        board[i as usize][j as usize] = '0';

        let r = Self::dfs(board, &word[1..], i - 1, j)
            || Self::dfs(board, &word[1..], i + 1, j)
            || Self::dfs(board, &word[1..], i, j - 1)
            || Self::dfs(board, &word[1..], i, j + 1);

        if !r {
            // undo mark as visited
            board[i as usize][j as usize] = old_c;
        }
        r
    }
}

pub mod tests {
    use super::Solution;

    pub fn test() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let word = "SEE".to_string();
        let r = Solution::exist(board, word);
        dbg!(r);
    }
}
