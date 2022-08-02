struct Solution;

impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let mut l = 0;
        let mut r: i32 = letters.len() as i32 - 1;

        while l <= r {
            let m = l + (r - l) / 2;
            if letters[m as usize] <= target {
                l = m + 1;
            } else {
                r = m - 1;
            }
        }

        if l >= letters.len() as i32 {
            letters[0]
        } else {
            letters[l as usize]
        }
    }
}

pub mod tests {
    use super::Solution;

    pub fn test() {
        let letters = vec!['c', 'f', 'j'];
        let target = 'g';
        let result = Solution::next_greatest_letter(letters, target);
        dbg!(result);
    }
}
