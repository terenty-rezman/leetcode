use crate::my_list::ListNode;

struct Solution;

impl Solution {
    pub fn insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut node_a = head;
        let mut sorted: Option<Box<ListNode>> = None;

        'outer: while node_a.is_some() {
            let mut curr_a = node_a.unwrap();
            node_a = curr_a.next.take();

            let mut node_b = &mut sorted;
            while node_b.is_some() {
                if curr_a.val >= node_b.as_ref().unwrap().val {
                    curr_a.next = node_b.take();
                    node_b.replace(curr_a);
                    continue 'outer;
                }
                node_b = &mut node_b.as_mut().unwrap().next;
            }

            node_b.replace(curr_a);
        }

        // reverse the list
        let mut node = sorted;
        let mut result = None;
        while node.is_some() {
            let next = node.as_mut().unwrap().next.take();
            node.as_mut().unwrap().next = result;
            result = node;
            node = next;
        }

        result
    }
}

pub mod tests {
    use super::Solution;
    use crate::my_list::{create_list, print_list};

    pub fn test() {
        let list = create_list(&[4, 2, 1, 3]);
        let r = Solution::insertion_sort_list(list);
        print_list(&r);
    }
}
