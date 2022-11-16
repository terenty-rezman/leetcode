use crate::my_list::{ListNode};

struct Solution;

impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        // count nodes
        let mut count = 0;
        let mut node = &*head;
        while let Some(n) = node.as_ref() {
            node = &n.next;
            count += 1;
        }

        // find split node ptr at the middle
        count /= 2;
        let mut split_node = &mut *head;
        while split_node.is_some() {
            if count == 1 {
                break;
            }
            count -= 1;
            split_node = &mut split_node.as_mut().unwrap().next;
        }

        let second_half = split_node.as_mut().unwrap().next.take();

        let mut fake_node = Some(Box::new(ListNode::new(0)));
        let mut result_tail = &mut fake_node;
        let mut first_half = &mut *head; 

        // merge two halfs
        while head.is_some() && second_half.is_some() {

        }

    }
}

pub mod tests {
    use super::Solution;
    use crate::my_list;

    pub fn test() {
       let mut list = my_list::create_list(&[1, 2, 3, 4, 5]);
       Solution::reorder_list(&mut list);
       my_list::print_list(&list);
    }
}
