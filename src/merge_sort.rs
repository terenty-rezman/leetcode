struct Solution;

impl Solution {
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        Solution::merge_sort(&nums)
    }

    fn merge_sort(nums: &[i32]) -> Vec<i32> {
        if nums.len() <= 1 {
            return nums.to_vec();
        }

        let m = nums.len() / 2;

        let a = Solution::merge_sort(&nums[0..m]);
        let b = Solution::merge_sort(&nums[m..nums.len()]);

        Solution::merge(&a, &b)
    }

    fn merge(a: &[i32], b: &[i32]) -> Vec<i32> {
        let mut res = Vec::new();
        res.resize(a.len() + b.len(), 0);

        let mut i = 0;
        let mut j = 0;
        let mut k = 0;

        while i < a.len() && j < b.len() {
            if a[i] < b[j] {
                res[k] = a[i];
                i += 1;
            } else {
                res[k] = b[j];
                j += 1;
            }
            k += 1;
        }

        while i < a.len() {
            res[k] = a[i];
            i += 1;
            k += 1;
        }

        while j < b.len() {
            res[k] = b[j];
            j += 1;
            k += 1
        }

        res
    }
}

pub mod tests {
    use super::Solution;

    pub fn test() {
        let nums = vec![5, 2, 3, 1];
        let r = Solution::sort_array(nums);
        dbg!(r);
    }
}
