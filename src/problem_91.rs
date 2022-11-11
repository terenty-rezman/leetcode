struct Solution;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let mut memo = vec![-1; s.len()];
        Self::dp(s.as_bytes(), 0, &mut memo)
    }

    fn dp(s: &[u8], pos: usize, memo: &mut Vec<i32>) -> i32 {
        if s.is_empty() {
            return 1;
        }

        if s[0] == b'0' {
            return 0;
        }

        let mut count = 0;

        if memo[pos] != -1 {
            return memo[pos];
        }

        count += Self::dp(&s[1..], pos + 1, memo);

        if s.len() == 1 {
            return count;
        }

        if s[0] > b'2' {
            return count;
        }

        if s[0] == b'2' && s[1] > b'6' {
            return count;
        }

        count += Self::dp(&s[2..], pos + 2, memo);
        memo[pos] = count;
        count
    }
}

pub mod tests {
    use super::Solution;

    pub fn test() {
        let s = "2611055971756562".to_string();
        let r = Solution::num_decodings(s);
        dbg!(r);
    }
}
