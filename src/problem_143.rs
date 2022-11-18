use crate::my_list::ListNode;

struct Solution;

impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        if head.is_none() {
            return;
        }

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
            if count <= 1 {
                break;
            }
            count -= 1;
            split_node = &mut split_node.as_mut().unwrap().next;
        }

        let mut second_half = split_node.as_mut().unwrap().next.take();

        // reverse second half
        let mut reversed_second_half = None;
        while second_half.is_some() {
            let temp_next = second_half.as_mut().unwrap().next.take();
            second_half.as_mut().unwrap().next = reversed_second_half.take();
            reversed_second_half = second_half.take();
            second_half = temp_next;
        }

        second_half = reversed_second_half;

        let mut fake_node = Some(Box::new(ListNode::new(0)));
        let mut result_tail = &mut fake_node;
        let mut first_half = head.take();

        // merge two halfs
        while first_half.is_some() && second_half.is_some() {
            std::mem::swap(result_tail, &mut first_half);
            first_half = result_tail.as_mut().unwrap().next.take();
            result_tail = &mut result_tail.as_mut().unwrap().next;

            std::mem::swap(result_tail, &mut second_half);
            second_half = result_tail.as_mut().unwrap().next.take();
            result_tail = &mut result_tail.as_mut().unwrap().next;
        }

        // add nodes left in first half
        if first_half.is_some() {
            result_tail.replace(first_half.take().unwrap());
        }

        // add nodes left in second half
        if second_half.is_some() {
            result_tail.replace(second_half.take().unwrap());
        }

        head.replace(fake_node.take().unwrap());
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
