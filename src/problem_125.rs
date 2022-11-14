struct Solution;

impl Solution {
    pub fn is_palindrome(mut s: String) -> bool {
        if s.is_empty() {
            return true;
        }
        
        s.make_ascii_lowercase();
        let s = s.as_bytes();
        
        let mut l = 0;
        let mut r = s.len() - 1; 
        
        while l < r {
            let ls = s[l];
            if !(b'a'..=b'z').contains(&ls) &&
               !(b'0'..=b'9').contains(&ls) 
            {
                l += 1;
                continue;
            }
            
            let rs = s[r];
            if !(b'a'..=b'z').contains(&rs) &&
               !(b'0'..=b'9').contains(&rs) 
            {
                r -= 1;
                continue;
            }
            
            if ls != rs {
               return false; 
            }

            l += 1;
            r -= 1;
        }
        
        true
    }
}

pub mod tests {
    use super::Solution;

    pub fn test() {
        let r = Solution::is_palindrome("A man, a plan, a canal: Panama".to_string());
        dbg!(r);
    }
}
