use std::cmp::max;
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }

        let mut found: HashMap<char, usize> = HashMap::new();
        let chars: Vec<char> = s.chars().collect();
        let mut l = 0;
        let mut r = 1;
        let mut max_len: i32 = 1;

        found.insert(chars[l], l);

        while r < chars.len() {
            let c = chars[r];

            if found.contains_key(&c) {
                let prev_pos = found.get(&c).unwrap() + 1;
                if prev_pos > l {
                    l = prev_pos;
                }
            }

            found.insert(c, r);
            max_len = max(max_len, r as i32 - l as i32 + 1);
            r += 1;
        }

        max_len
    }
}

pub mod tests {
    use super::Solution;

    pub fn test() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring("bbbbb".to_string()),
            1
        );
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string()),
            3
        );
        assert_eq!(Solution::length_of_longest_substring("abba".to_string()), 2);
    }
}
