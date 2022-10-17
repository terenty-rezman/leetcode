struct Solution;

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut cnt = [0_usize; 3];

        for &n in nums.iter() {
            cnt[n as usize] += 1;
        }

        let mut j = 0;
        for (i, &c) in cnt.iter().enumerate() {
            for _ in 0..c {
                nums[j] = i as i32;
                j += 1;
            }
        }
    }
}

pub mod tests {
    use super::Solution;

    pub fn test() {
        let mut nums = vec![2, 0, 2, 1, 1, 0];
        Solution::sort_colors(&mut nums);
        dbg!(nums);
    }
}
