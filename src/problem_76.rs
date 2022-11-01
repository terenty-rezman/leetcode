use std::collections::{HashSet, HashMap};
use std::iter::FromIterator;

struct Solution;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let mut not_occured: HashSet<char> = HashSet::from_iter(t.chars());
        let mut count_occured: HashMap<char, usize> = HashMap::new();
        let mut count_target = HashMap::new();
        
        // count chars in target string
        for c in t.chars() {
            let counter = count_target.entry(c).or_insert(0);
            *counter += 1;
        }
        
        let s: Vec<char> = s.chars().collect();
        let mut l = 0;
        let mut r = 0;
        let mut result_r = 0;
        let mut result_l = 0;
        let mut result_len = s.len() + 1; 
        
        while r <= s.len() {
            if not_occured.is_empty() {
                // move l to left reducing result str
                while l < r {
                    let c = s[l];
                    if !count_target.contains_key(&c) {
                        l += 1;
                        continue;
                    }  
                    else if count_occured[&c] > count_target[&c] 
                    {
                        count_occured.entry(c).and_modify(|e| *e -= 1);
                        l += 1;
                        continue;
                    }
                    break;
                }
                
                if r - l < result_len {
                    result_l = l;
                    result_r = r;
                    result_len = r - l;
                }
            }

            if r == s.len() {
                break;
            }
            
            // move r - expand to the right
            let c = s[r];
            if count_target.contains_key(&c) {
                *count_occured.entry(c).or_insert(0) += 1;
                if count_occured[&c] == count_target[&c] {
                    not_occured.remove(&c);
                }
            }
            r += 1;
        }
        if not_occured.is_empty() {
            return s[result_l..result_r].iter().collect();
        }
        
        "".to_string()
    }
}

pub mod tests {
    use super::Solution;

    pub fn test() {
        let r = Solution::min_window("a".to_string(), "a".to_string());
        dbg!(r);
    }
}