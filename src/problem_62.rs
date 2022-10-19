struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut grid = vec![vec![0; n as usize + 1]; m as usize + 1];
        grid[0][1] = 1;

        for i in 1..n as usize + 1 {
            for j in 1..m as usize + 1 {
                grid[j][i] = grid[j - 1][i] + grid[j][i - 1];
            }
        }

        grid[m as usize][n as usize]
    }
}

pub mod tests {
    use super::Solution;

    pub fn test() {
        let res = Solution::unique_paths(4, 7);
        dbg!(res);
    }
}
