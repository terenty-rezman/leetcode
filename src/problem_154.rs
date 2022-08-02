struct Solution;

static mut steps_count: usize = 0;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            unsafe {
                steps_count += 1;
            }
            return nums[0];
        }

        if nums.len() == 2 {
            unsafe {
                steps_count += 1;
            }
            return std::cmp::min(nums[0], nums[1]);
        }

        let mut l = 0;
        let mut r = nums.len() - 1;

        while l + 1 < r {
            unsafe {
                steps_count += 1;
            }

            let m = l + (r - l) / 2;

            if nums[m] == nums[r] {
                let a = Solution::find_min(nums[m..r].to_vec());
                let b = Solution::find_min(nums[l..m].to_vec());
                return std::cmp::min(a, b);
            }

            if nums[m] > nums[r] {
                l = m;
            } else {
                r = m;
            }
        }

        std::cmp::min(nums[l], nums[r])
    }
}

pub mod tests {
    use crate::problem_154::steps_count;

    use super::Solution;

    pub fn test() {
        let nums = vec![
            3, 3, 3, 3, 3, 1, 1, 1, 1, 1, 1, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
        ];

        dbg!(nums.len());

        let result = Solution::find_min(nums);

        unsafe {
            dbg!(steps_count);
        }

        dbg!(result);
    }
}
