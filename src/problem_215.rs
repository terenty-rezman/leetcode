struct Solution;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        if nums.len() == 1 {
            return nums[0];
        }
        let len = nums.len() as i32;
        let r = nums.len() - 1;
        Solution::kth_largest(&mut nums, (len - k) as usize, 0, r as i32)
    }
    
    fn kth_largest(nums:&mut [i32], k: usize, l: i32, r: i32) -> i32 {
        if r - l <= 0 {
            return nums[k];
        }
        
        let x = nums[r as usize];
        let mut i = l as i32 - 1;
        let mut j = l;

        while j <= r - 1 {
            if nums[j as usize] < x {
                i += 1;
                nums.swap(i as usize, j as usize);
            }
            j += 1;
        }

        nums.swap((i + 1) as usize, r as usize);

        let m = i + 1;

        let len = nums.len() as i32;

        if k as i32 == m  {
            return nums[k];
        }
        else if (k as i32) < m {
            Solution::kth_largest(nums, k, l, m - 1)
        } else {
            Solution::kth_largest(nums, k, m + 1, r)
        }
    }
}

pub mod tests {
    use super::Solution;

    pub fn test() {
        let nums = vec![3,2,1,5,6,4];
        let r = Solution::find_kth_largest(nums, 2);
        dbg!(r);
    }
}
