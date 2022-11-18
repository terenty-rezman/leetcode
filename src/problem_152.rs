struct Solution;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut cur_max = nums[0];
        let mut cur_min = nums[0];
        let mut max = nums[0];

        for &n in &nums[1..] {
            let tmp = n.max(n * cur_max).max(n * cur_min);
            cur_min = n.min(n * cur_max).min(n * cur_min);
            cur_max = tmp;

            max = max.max(cur_max);
        }

        max
    }
}

pub mod tests {
    use super::Solution;

    pub fn test() {
        let r = Solution::max_product(vec![1, 2, -3, 0, -4, -5]);
        dbg!(r);
    }
}
