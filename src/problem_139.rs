struct Solution;

use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let word_set: HashSet<&str> = word_dict.iter().map(|e| e.as_str()).collect();
        let mut memo: HashMap<&str, bool> = HashMap::new();
        Self::dp(&s, &word_set, &mut memo)
    }

    fn dp<'a>(s: &'a str, word_set: &HashSet<&str>, memo: &mut HashMap<&'a str, bool>) -> bool {
        for i in 1..s.len() + 1 {
            let word = &s[0..i];

            if word_set.contains(&word) {
                let rest = &s[i..];

                if rest.is_empty() {
                    return true;
                }

                if memo.contains_key(&rest) {
                    if *memo.get(&rest).unwrap() {
                        return true;
                    }
                } else {
                    let success = Self::dp(rest, word_set, memo);
                    memo.insert(rest, success);
                    if success {
                        return true;
                    }
                }
            }
        }

        false
    }
}

pub mod tests {
    use super::Solution;

    pub fn test() {
        let r = Solution::word_break(
            "aaaaaaa".to_string(),
            vec!["aaaa".to_string(), "aaa".to_string()],
        );
        dbg!(r);
    }
}
