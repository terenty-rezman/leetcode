struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        // use std::iter::FromIterator;
        use std::collections::HashSet;
        
        let mut set: HashSet<i32> = HashSet::from_iter(nums.iter().cloned());
        let mut max_len = 0; 
        
        for n in nums {
            let mut len = 0;
            let mut number = n;
            
            if set.contains(&(number + 1)) {
                while set.contains(&number) {
                    set.remove(&number);
                    len += 1;
                    number += 1;
                }
            } else {
                while set.contains(&number) {
                    set.remove(&number);
                    len += 1;
                    number -= 1;
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
        let r = Solution::longest_consecutive(vec![1,2,0,1]);
        dbg!(r);
        

    }
}
