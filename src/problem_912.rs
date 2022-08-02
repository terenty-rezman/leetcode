struct Solution;

impl Solution {
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        quick_sort(&mut nums);
        nums
    }

    pub fn sort_array_merge(nums: Vec<i32>) -> Vec<i32> {
        merge_sort(&nums)
    }
}

fn partition(s: &mut [i32]) -> usize {
    let mut j: i32 = -1;

    for i in 0..s.len() {
       if s[i] <= s[s.len() - 1] {
           j += 1;
           s.swap(i, j as usize);
       } 
    }

    j as usize
}


fn quick_sort(s: &mut [i32]) {
   if s.len() <= 1 {
       return;
   } 

   let p = partition(s);
   quick_sort(&mut s[..p]);
   quick_sort(&mut s[p..]);
}

fn merge_sort(nums: &[i32]) -> Vec<i32> {
    if nums.len() <= 1 {
        return nums.to_vec();
    }
    
    let m = nums.len() / 2;
    
    let a = merge_sort(&nums[0..m]);
    let b = merge_sort(&nums[m..nums.len()]);
    
    merge(&a, &b)
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


pub mod tests {
    use super::Solution;
    use std::time::{Duration, Instant};
    use rand::{thread_rng, Rng, Fill};

    pub fn test() {
        let size = 100_000;
        let mut v = vec![i32::default(); size];
        rand::thread_rng().fill(&mut v[..]);

        let mut v2 = v.clone();

        let now = Instant::now();
        v = Solution::sort_array(v);
        println!("{}", now.elapsed().as_secs_f32());

        let now = Instant::now();
        v2 = Solution::sort_array_merge(v2);
        println!("{}", now.elapsed().as_secs_f32());
    }
}
