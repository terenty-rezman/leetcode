struct Solution;

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n == 0 {
            return 1.0;
        }
        if n == 1 {
            return x;
        }
        if n == -1 {
            return 1.0 / x;
        }

        if n > 0 {
            if n % 2 == 0 {
                let r = Solution::my_pow(x, n / 2);
                return r * r;
            } else {
                let r = Solution::my_pow(x, (n - 1) / 2);
                return x * r * r;
            }
        }
        else {
            if n % 2 == 0 {
                let r = Solution::my_pow(x, n / 2);
                return r * r;
            } else {
                let r = Solution::my_pow(x, (n + 1) / 2);
                return 1.0 / x * r * r;
            }
        }
    }
}

pub mod tests {
    use super::Solution;

    pub fn test() {
        let x = 2.0;
        let n = -3;
        let result = Solution::my_pow(x, n);
        dbg!(result);
    }
}
