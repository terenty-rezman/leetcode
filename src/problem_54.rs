struct Solution;

impl Solution {
    pub fn spiral_order(mut matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let steps = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut result = vec![];
        let mut to_visit = matrix[0].len() * matrix.len();
        let len_i: i32 = matrix.len() as i32;
        let len_j: i32 = matrix[0].len() as i32;

        let mut i: i32 = 0;
        let mut j: i32 = -1;
        let mut step = steps.iter().cycle();
        let &(mut step_i, mut step_j) = step.next().unwrap();

        while to_visit > 0 {
            let next_i = i + step_i;
            let next_j = j + step_j;

            if next_i >= 0 && next_i < len_i && next_j >= 0 && next_j < len_j {
                let ii = next_i as usize;
                let jj = next_j as usize;
                if matrix[ii][jj] != 101 {
                    result.push(matrix[ii][jj]);
                    matrix[ii][jj] = 101;
                    to_visit -= 1;
                    i = next_i;
                    j = next_j;
                } else {
                    (step_i, step_j) = *step.next().unwrap();
                }
            } else {
                (step_i, step_j) = *step.next().unwrap();
            }
        }

        result
    }
}

pub mod tests {
    use super::Solution;

    pub fn test() {
        let res = Solution::spiral_order(vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
        ]);

        dbg!(res);
    }
}
