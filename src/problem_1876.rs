use std::collections::HashMap;
use std::u8;

struct Solution;

struct CountMap {
    map: HashMap<u8, i32>,
}

impl CountMap {
    fn new() -> Self {
        Self { map: HashMap::new() }
    }

    fn add(&mut self, item: u8) {
        if let Some(count) = self.map.get_mut(&item) {
            *count += 1;
        } else {
            self.map.insert(item, 1);
        }
    }

    fn remove(&mut self, item: &u8) {
        if let Some(count) = self.map.get_mut(item) {
            *count -= 1;

            if *count == 0 {
                self.map.remove(item);
            }
        }
    }

    fn len(&self) -> usize {
        self.map.len()
    }
}

impl Solution {
    pub fn count_good_substrings(s: String) -> i32 {
        if s.len() < 3 {
            return 0;
        }

        let mut map = CountMap::new();

        for item in s.bytes().take(2) {
            map.add(item);
        }

        let first_c = s.bytes();
        let third_c = s.bytes().skip(2);
        let mut remove_c = 0_u8;
        let mut count = 0;

        for (f_c, t_c) in first_c.zip(third_c) {
            map.remove(&remove_c);
            map.add(t_c);

            if map.len() == 3 {
                count += 1;
            }
            remove_c = f_c;
        }

        count
    }
}

pub mod tests {
    use super::Solution;

    pub fn test() {
        dbg!(Solution::count_good_substrings("aababcabc".to_string()));
    }
}
