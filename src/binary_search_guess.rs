fn guess(x: i32) -> i32 {
    let guessed = 1702766719;

    if x < guessed {
        1
    } else if x > guessed {
        -1
    }
    else {
        0
    }
}

struct Solution;

impl Solution {
    fn guessNumber(n: i32) -> i32 {
        let mut l: i64 = -1;
        let mut r: i64 = n as i64 + 1;
        
        loop {
            let m = (l + r) / 2;
            let g = guess(m as i32);
            
            if g == 0 {
                return m as i32;
            } else if g == -1 {
                r = m as i64;
            } else if g == 1 {
                l = m as i64;
            }
        }

        unreachable!();
    }
}

pub mod tests {
    use super::Solution;

    pub fn test() {
        let n = 2126753390;
        let result = Solution::guessNumber(n);
        dbg!(result);
    }
}
