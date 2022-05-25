use core::num;

struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut r = nums.len() - 1;
        let mut l = 0;
        let mut min_val = 0;
        let mut min_i = 0;
        
        loop {
            let m_i = (l + r) / 2;
            let m_val = nums[m_i];
            
            if nums[l] <= m_val && m_val <= nums[r] {
                min_val = nums[l];
                min_i = l;
                break;
            }

            if r - l <= 1 {
                if nums[l] < nums[r] {
                    min_i = l;
                    min_val = nums[l];
                } else {
                    min_i = r;
                    min_val = nums[r];
                }
                break;
            }
            
            if nums[m_i] < nums[l] {
                r = m_i;
            } else if nums[m_i] > nums[r] {
                l = m_i;
            }
        }

        let offset = min_i;

        let mut l = 0;
        let mut r: i32 = (nums.len() - 1) as i32;
        let mut idx: i32 = -1;

        while l <= r {
            dbg!(l);
            dbg!(r);
            let m = (l + r) / 2;
            let m_real: usize = (m as usize + offset) % nums.len() as usize;
            if nums[m_real] == target {
                idx = m_real as i32;
                break;
            } else if target < nums[m_real] {
                r = m - 1;
            } else {
                l = m + 1;
            }
        }

        idx as i32
    }
}

pub mod tests {
    use super::Solution;

    pub fn test() {
        let result = Solution::search(vec![6, 7, 8, 1, 2, 3, 4, 5], 8);
        dbg!(result);
    }
}
