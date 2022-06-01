struct Solution;

impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let mut l = 0;
        let mut r: i64 = num as i64;
        while l <= r {
            let m = l + (r - l) / 2;
            let v = m * m;
            if v == num as i64 {
                return true;
            } else if v < num as i64 {
                l = m + 1;
            } else {
                r = m - 1;
            }
        }
        false
    }

    pub fn wtf(mut num: i32) -> bool {
        let mut i = 1;
        while 0 < num {
            num -= i;
            i += 2;  
        }
        num == 0
    }
}

pub mod tests {
    use super::Solution;

    pub fn test() {
        let num = 1;
        let result = Solution::is_perfect_square(num);
        let result_1 = Solution::wtf(num);
        dbg!(result);
        dbg!(result_1);
    }
}