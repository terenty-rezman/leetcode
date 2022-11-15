struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        // use std::iter::FromIterator;
        use std::collections::HashSet;

        let mut set: HashSet<i32> = HashSet::from_iter(nums.iter().cloned());
        let mut max_len = 0;

        for n in nums {
            let mut len = 0;

            set.remove(&n);
            len += 1;

            if set.contains(&(n + 1)) {
                let mut r = n + 1;
                while set.contains(&r) {
                    set.remove(&r);
                    len += 1;
                    r += 1;
                }
            }

            if set.contains(&(n - 1)) {
                let mut l = n - 1;
                while set.contains(&l) {
                    set.remove(&l);
                    len += 1;
                    l -= 1;
                }
            }

            max_len = std::cmp::max(len, max_len);
        }

        max_len
    }
}

pub mod tests {
    use super::Solution;

    pub fn test() {
        let r = Solution::longest_consecutive(vec![1, 2, 0, 1]);
        dbg!(r);
    }
}
