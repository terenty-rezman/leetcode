use std::fmt;

#[derive(Debug)]
struct ListNode {
    data: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(data: i32) -> Self {
        ListNode { data, next: None }
    }
}

impl fmt::Display for ListNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.next {
            Some(node) => {
                write!(f, "{},", self.data);
                node.fmt(f)
            }
            None => write!(f, "{}", self.data)
        }
    }
}

fn create_list(array: &[i32]) -> Option<Box<ListNode>> {
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
