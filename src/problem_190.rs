struct Solution;

impl Solution {
    pub fn reverse_bits(mut x: u32) -> u32 {
        let mut result = 0;
        
        for _ in 0..32 {
            let r_bit = x & 0b1;
            x >>= 1; 
            
            result <<= 1;
            result |= r_bit;
        }
        
        result
    }
}

pub mod tests {
    use super::Solution;

    pub fn test() {
        let r = Solution::reverse_bits(0b00000010100101000001111010011100);
        println!("{:b}", r);
    }
}
