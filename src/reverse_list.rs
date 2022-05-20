use crate::my_list::ListNode;

struct Solution;

impl Solution {
    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut result: Option<Box<ListNode>> = None;
        
        while let Some(curr) = &mut head {
            std::mem::swap(&mut curr.next, &mut result);
            std::mem::swap(&mut head, &mut result);
        }

        result
    }
}

pub mod tests {
    use crate::my_list::create_list;
    use super::Solution;

    pub fn test() {
        let head = create_list(&[1, 2, 3, 4, 5]);
        let result = Solution::reverse_list(head);
        dbg!(result);
    }
}
