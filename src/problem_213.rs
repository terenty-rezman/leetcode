struct Solution;

impl Solution {
    pub fn rob(mut nums: Vec<i32>) -> i32 {
        if nums.len() <= 3 {
            return nums.into_iter().reduce(std::cmp::max).unwrap_or(0);
        }
        
        fn max_rob(nums: &[i32]) -> i32 {
            let mut max_3 = nums[0];
            let mut max_2 = nums[1];
            let mut max_1 = nums[2] + nums[0];
            
            for i in 3..nums.len() {
                let max_cur = nums[i] + std::cmp::max(max_3, max_2);
                
                max_3 = max_2;
                max_2 = max_1;
                max_1 = max_cur;
            }
            
            std::cmp::max(max_1, max_2)
        }

        if nums.len() == 4 {
            let _3 = nums[0];
            let _2 = nums[1];
            let _1 = nums[2];
            let _0 = nums[3];

            return (_3 + _1).max(_2 + _0);
        }

        let m = nums.len();
        let max_1 = max_rob(&nums[..m - 1]);

        let max_2 = max_rob(&nums[1..]);
        max_1.max(max_2)
    }
}

pub mod tests {
    use super::Solution;

    pub fn test() {
        let r = Solution::rob(vec![4,1,2,7,5,3,1]);
        dbg!(r);
    }
}

