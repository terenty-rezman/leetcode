struct Solution;

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.is_empty() {
            return vec![];
        }

        let mut prev_start = intervals[0][0];
        let mut prev_end = intervals[0][1];
        let mut res = vec![];
        
        for i in intervals {
            let cur_start = i[0];
            let cur_end = i[1];
            
            if cur_start <= prev_end {
                prev_end = cur_end;
            } else {
                res.push(vec![prev_start, prev_end]);
                prev_start = cur_start;
                prev_end = cur_end;
            }
        }

        res.push(vec![prev_start, prev_end]);
        
        res
    }
}

pub mod tests {
    use super::Solution;

    pub fn test() {
        let res = Solution::merge(vec![
            vec![1, 4],
            vec![0, 5]
        ]);

        dbg!(res);
    }
}