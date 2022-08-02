struct Solution;

use std::collections::HashSet;
use std::iter::FromIterator;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let nums1: HashSet<i32> = HashSet::from_iter(nums1);
        let nums2: HashSet<i32> = HashSet::from_iter(nums2);
        nums1.intersection(&nums2).cloned().collect()
    }
}

pub mod tests {
    use super::Solution;

    pub fn test() {
        let nums1 = vec![1, 2, 3, 4];
        let nums2 = vec![3, 3, 5];

        dbg!(Solution::intersection(nums1, nums2));
    }
}
