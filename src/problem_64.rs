struct Solution;

impl Solution {
    pub fn min_path_sum(mut grid: Vec<Vec<i32>>) -> i32 {
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if i == 0 && j == 0 {
                    continue;
                }

                let t = i as i32 - 1;
                let l = j as i32 - 1;

                let t_val = if t < 0 { std::i32::MAX } else { grid[t as usize][j] };

                let l_val = if l < 0 { std::i32::MAX } else { grid[i][l as usize] };

                grid[i][j] += std::cmp::min(l_val, t_val);
            }
        }

        *grid.last().unwrap().last().unwrap()
    }
}

pub mod tests {
    use super::Solution;

    pub fn test() {
        let v = vec![
            vec![1, 2, 3],
            vec![1, 2, 3],
            vec![1, 1, 3]
        ];

        let res = Solution::min_path_sum(v);
        dbg!(res);
    }
}
