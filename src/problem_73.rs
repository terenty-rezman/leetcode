struct Solution;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        if matrix.is_empty() {
            return;
        }
        
        let mut first_row = 1;
        let m = matrix.len();
        let n = matrix[0].len();
        
        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == 0 {
                    if i == 0 {
                        first_row = 0;
                    } else {
                        matrix[i][0] = 0;
                    }
                    matrix[0][j] = 0;
                }
            }
        }
        
        // zerofy columns except first one
        for j in 1..n {
            if matrix[0][j] == 0 {
                for i in 0..m {
                    matrix[i][j] = 0;
                }
            }
        }
        
        // zerofy rest rows
        for i in 1..m {
            if matrix[i][0] == 0 {
                for j in 0..n {
                   matrix[i][j] = 0 
                }
            }
        }
        
        // zefory first column
        if matrix[0][0] == 0 {
            for i in 0..m {
                matrix[i][0] = 0;
            }
        }

        // zerofy first row
        if first_row == 0 {
           for j in 0..n {
               matrix[0][j] = 0;
           } 
        } 
    }
}

pub mod tests {
    use super::Solution;

    pub fn test() {
        let mut input = vec![
            vec![-4,-2147483648,6,-7,0],
            vec![-8,6,-8,-6,0],
            vec![2147483647,2,-9,-6,-10]
        ]; 
        Solution::set_zeroes(&mut input);
        dbg!(input);
    }
}
