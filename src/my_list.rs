use std::fmt;

#[derive(Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

impl fmt::Display for ListNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.next {
            Some(node) => {
                let _ = write!(f, "{},", self.val);
                node.fmt(f)
            }
            None => write!(f, "{}", self.val),
        }
    }
}

pub fn create_list(array: &[i32]) -> Option<Box<ListNode>> {
    if array.is_empty() {
        None
    } else {
        let mut head = ListNode::new(array[0]);
        let mut curr_node = &mut head;

        for &v in array.iter().skip(1) {
            let next_node = ListNode::new(v);
            curr_node.next = Some(Box::new(next_node));
            curr_node = curr_node.next.as_mut().unwrap();
        }

        Some(Box::new(head))
    }
}

pub fn print_list(head: &Option<Box<ListNode>>) {
    let mut node = head;
    while let Some(nd) = node {
        print!("{} ", nd.val);
        node = &nd.next;
    }
}

pub mod tests {
    use super::*;

    #[allow(dead_code)]
    pub fn test() {
        let mut head = create_list(&[1, 2, 3]);

        let mut curr_node = &mut head;
        let n = 3;
        let mut i = 1;

        while let Some(node) = curr_node.as_mut() {
            // while curr_node.is_some() {
            if i == n {
                break;
            }
            curr_node = &mut node.next;
            // curr_node = &mut curr_node.as_mut().unwrap().next;
            i += 1;
        }

        // let head = curr_node.as_mut().unwrap();
    }
}
