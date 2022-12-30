struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut memo = vec![vec![-1; nums.len()]; nums.len()];

        fn dp(nums: &[i32], pos: usize, prev_num_idx: i32, memo: &mut [Vec<i32>]) -> i32 {
            if pos >= nums.len() {
                return 0;
            }

            let mut continue_take = 0;
            let mut start_take = 0;
            if prev_num_idx > 0 {
                if nums[pos] > nums[prev_num_idx as usize] {
                    continue_take = dp(nums, pos + 1, pos as i32, memo);
                    continue_take += 1;
                } 
            } else {
                start_take = dp(nums, pos + 1, pos as i32, memo);
                start_take += 1;
            }

            let dont_take = dp(nums, pos + 1, prev_num_idx, memo); 

            std::cmp::max(dont_take, continue_take).max(start_take)
        }

        dp(&nums, 0, -1, &mut memo)
    }
}

pub mod tests {
    use super::Solution;

    pub fn test() {
        let r = Solution::length_of_lis(vec![10,9,2,5,3,7,101,18]);
        dbg!(r);
    }
}
