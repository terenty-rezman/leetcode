struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.is_empty() {
            return 0;
        }

        let haystack = haystack.as_bytes();
        let needle = needle.as_bytes();
        let mut match_i = 0;
        let mut j = 0;
        let mut match_len = 0;

        while j < haystack.len() {
            if haystack[j] == needle[match_len] {
                if match_len == 0 {
                    match_i = j;
                }
                match_len += 1;
            } else {
                if match_len > 0 {
                    j = match_i;
                }
                match_len = 0;
            }

            if match_len == needle.len() {
                return (match_i) as i32;
            }

            j += 1;
        }

        -1
    }
}

pub mod tests {
    use super::Solution;

    pub fn test() {
        let haystack = "mississippi".to_owned();
        let needle = "pi".to_owned();
        let r = Solution::str_str(haystack, needle);
        dbg!(r);
    }
}
