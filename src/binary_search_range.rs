struct Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut l = 0;
        let mut r = nums.len() - 1;
        
        if nums.is_empty() {
            return vec![-1, -1];
        }
        
        while l + 1 < r {
            let m = l + (r - l) / 2;
            if nums[m] <= target {
                l = m;
            } else {
                r = m;
            }
        }

        let end = {
            if nums[r] == target {
                r as i32
            } else if nums[l] == target {
                l as i32
            } else {
                -1
            }
        };

        l = 0;
        r = nums.len() - 1;
        while l + 1 < r {
            let m = l + (r - l) / 2;
            if nums[m] >= target {
                r = m;
            } else {
                l = m;
            }
        }

        let start = {
            if nums[l] == target {
                l as i32
            } else if nums[r] == target {
                r as i32
            } else {
                -1
            }
        };
        
        vec![start, end]
       }
}

pub mod tests {
    use super::Solution;

    pub fn test() {
        let input = vec![1, 3];
        let target = 1;
        let result = Solution::search_range(input, target);
        dbg!(result);
    }

}