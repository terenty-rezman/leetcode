struct Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut number_count_map = HashMap::new();
        // number: count mapping
        for n in nums {
            let e = number_count_map.entry(n).or_insert(0);
            *e += 1;
        }

        // count: number mapping
        let mut number_count_vec: Vec<(i32, usize)> = number_count_map.into_iter().collect();

        let mut ps = 0;
        let mut pe = number_count_vec.len();
        // partition sorts in ascending mode so we need k elements from the end
        let q = number_count_vec.len() - k as usize;
        loop {
            let p = Solution::partition_by_key(&mut number_count_vec[ps..pe], |a| &a.1);
            if ps + p == q {
                break;
            }

            if q > p + ps {
                ps = ps + p;
            } else {
                pe = ps + p;
            }
        }

        let mut result = Vec::with_capacity(k as usize);
        for c in &number_count_vec[q as usize..] {
            result.push(c.0);
        }
        result
    }

    fn partition_by_key<T, F, K>(nums: &mut [T], f: F) -> usize
    where
        F: Fn(&T) -> &K,
        K: PartialOrd
    {
        let mut i: i32 = -1;
        for j in 0..nums.len() {
            let el = &nums[j];
            let el = f(el);

            let last = nums.last().unwrap();
            let last = f(last);

            if *el <= *last {
                i += 1;
                nums.swap(i as usize, j);
            }
        }

        i as usize
    }
}

pub mod tests {
    use super::Solution;

    pub fn test() {
        let input = vec![4,1,-1,2,-1,2,3];
        let r = Solution::top_k_frequent(input, 2);
        // let mut input = vec![3,1,2,1,3,2];
        // let r = Solution::partition_by_key(&mut input, |x| x);
        dbg!(r);
    }
}
