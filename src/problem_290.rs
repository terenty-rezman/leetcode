struct Solution;
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let mut map: HashMap<char, &str> = HashMap::new();
        let mut already_mapped: HashSet<&str> = HashSet::new();
        let mut chars = pattern.chars().peekable();
        let mut words = s.split_whitespace().peekable();

        loop {
            if chars.peek().is_none() || words.peek().is_none() {
                break;
            }

            let c = chars.next().unwrap();
            let w = words.next().unwrap();

            if already_mapped.contains(w) && !map.contains_key(&c) {
                return false;
            }

            already_mapped.insert(w);
            let &mut e = map.entry(c).or_insert(w);
            if w != e {
                return false;
            }
        }

        if chars.next().is_some() || words.next().is_some() {
            return false;
        }

        true
    }
}

pub mod tests {
    use super::Solution;

    pub fn test() {
        let pattern = "he".to_string();
        let s = "unit".to_string();
        let r = Solution::word_pattern(pattern, s);
        dbg!(r);
    }
}
