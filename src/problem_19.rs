#![allow(unused)]
#[derive(Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

fn count_nodes(head: &Option<Box<ListNode>>) -> usize {
    let mut count = 0;
    let mut next_node = head;
    while let Some(node) = next_node {
        next_node = &node.next;
        count += 1;
    }
    count
}

fn get_nth_node_mut(head: &mut Option<Box<ListNode>>, n: usize) -> Option<&mut ListNode> {
    let mut count = 0;
    let mut next_node: Option<&mut ListNode> = head.as_mut().map(|node| &mut **node);

    while next_node.is_some() {
        if count == n {
            break;
        }

        let next = next_node.take().unwrap();
        next_node = next.next.as_deref_mut();
        count += 1;
    }
    next_node
}

fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut head = head;
    let count = count_nodes(&head);
    let remove_number = (count as i32) - n;

    if remove_number == 0 {
        if head.is_some() {
            return head.unwrap().next;
        } else {
            return None;
        }
    }

    let parent = get_nth_node_mut(&mut head, (remove_number - 1) as usize).unwrap();

    let removed_node = parent.next.take();
    parent.next = removed_node.unwrap().next;
    head
}

pub fn test() {
    fn create_list(array: &[i32]) -> Option<Box<ListNode>> {
        if !array.is_empty() {
            let mut head = ListNode::new(array[0]);
            let mut node = &mut head;

            for &i in array.iter().skip(1) {
                let next = ListNode::new(i);
                node.next = Some(Box::new(next));
                node = node.next.as_mut().unwrap();
            }
            Some(Box::new(head))
        } else {
            None
        }
    }

    let head = create_list(&[1, 2, 3, 4]);

    let count = count_nodes(&head);

    // let result = remove_nth_from_end(head, 1);
    let result = remove_nth_from_end_leet(head, 2);
    dbg!(result);

    dbg!(count);
}

fn remove_nth_from_end_leet(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut pointer = &mut head;
    let mut len = 0;
    while let Some(p) = pointer {
        len += 1;
        pointer = &mut p.next;
    }
    len -= n;
    if len == 0 {
        head = head.unwrap().next;
    } else {
        let mut pointer = &mut head;
        while let Some(p) = pointer {
            if len == 1 {
                p.next = p.next.take().unwrap().next;
                break;
            } else {
                len -= 1;
                pointer = &mut p.next;
            }
        }
    }
    head
}
