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
        let mut count_number_map: HashMap<i32, i32> = HashMap::new();
        for (&k, &v) in number_count_map.iter() {
            count_number_map.insert(v, k);
        }
        let mut counts: Vec<i32> = count_number_map.keys().cloned().collect();

        let mut ps = 0; 
        let mut pe = counts.len(); 
        // partition sorts in ascending mode so we need k elements from the end
        let q = counts.len() - k as usize;
        loop {
            let p = Solution::partition(&mut counts[ps..pe]); 
            if p == q {
                break;
            }

            if p < q {
                ps = p;
            } else {
                pe = p;
            }
        }
        
        let mut result = Vec::with_capacity(k as usize);
        for c in &counts[q as usize..] {
            result.push(count_number_map[c]);
        }
        result
    }

    fn partition(nums: &mut [i32]) -> usize {
        let mut i = -1;
        for j in 0..nums.len() {
            if nums[j] <= *nums.last().unwrap() {
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
        let input = vec![1,2];
        let r = Solution::top_k_frequent(input, 2);
        // let mut input = vec![1,1,1,2,2,3];
        // let r = partition(&mut input);
        dbg!(r);
    }
}
