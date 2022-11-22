struct Solution;

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        use std::collections::{HashMap, HashSet};

        let mut course_map: HashMap<i32, Vec<i32>> = HashMap::new();

        for item in prerequisites {
            let (c, p) = (item[0], item[1]);
            course_map.entry(c).or_default().push(p);
        }

        fn dfs(
            c: i32,
            course_map: &HashMap<i32, Vec<i32>>,
            visited: &mut HashSet<i32>,
            memo: &mut HashSet<i32>,
        ) -> bool {
            if !course_map.contains_key(&c) {
                return true;
            }

            let courses = &course_map[&c];
            if visited.contains(&c) {
                return false;
            }

            visited.insert(c);

            for &c in courses {
                if memo.contains(&c) {
                    continue;
                }

                if !dfs(c, course_map, visited, memo) {
                    return false;
                }
            }

            visited.remove(&c);
            memo.insert(c);
            true
        }

        let mut visited = HashSet::new();
        let mut memo = HashSet::new();
        for c in course_map.keys().cloned().collect::<Vec<i32>>() {
            if !dfs(c, &course_map, &mut visited, &mut memo) {
                return false;
            }
        }

        true
    }
}

pub mod tests {
    use super::Solution;

    pub fn test() {
        let r = Solution::can_finish(2, vec![vec![1,4],vec![2,4],vec![3,1],vec![3,2]]);
        dbg!(r);
    }
}
