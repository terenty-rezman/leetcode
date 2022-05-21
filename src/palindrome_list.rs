use crate::my_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn is_palindrome(mut head: Option<Box<ListNode>>) -> bool {
        // count nodes
        let mut node = &head;
        let mut node_count = 0;
        while let Some(nd) = node.as_ref() {
            node_count += 1;
            node = &nd.next;
        }
        
        // find second half ref
        let mut second_half_i = node_count - (node_count / 2);
        let mut node = &mut head;
        while second_half_i > 0 {
            second_half_i -= 1;
            node = &mut node.as_mut().unwrap().next;
        }
        
        dbg!(second_half_i);
        
        // reverse second half
        let mut reverse_head: Option<Box<ListNode>> = None;
        let mut node = node.take();
        while node.is_some() {
            let mut next = node.as_mut().unwrap().next.take();
            let mut new_head = node.take();
            new_head.as_mut().unwrap().next = reverse_head.take();
            reverse_head = new_head;
            node = next;
        }
        
        let mut first = &head;
        let mut second = &reverse_head;
        while second.is_some() {
            if first.as_ref().unwrap().val != second.as_ref().unwrap().val {
                return false;
            }
            first = &first.as_ref().unwrap().next;
            second = &second.as_ref().unwrap().next;
        }
        
        true
    }
}


pub mod tests {
    use crate::my_list::create_list;
    use super::Solution;

    pub fn test() {
        let head = create_list(&[1, 2, 2, 1]);
        dbg!(Solution::is_palindrome(head));
    } 
}