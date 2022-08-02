struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

struct MyLinkedList {
    head: Option<Box<Node>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyLinkedList {
    fn new() -> Self {
        Self { head: None }
    }

    fn get(&self, index: i32) -> i32 {
        let mut count = index;
        let mut node = &self.head;

        while node.is_some() {
            if count <= 0 {
                break;
            }
            node = &node.as_ref().unwrap().next;
            count -= 1;
        }

        if node.is_some() && count == 0 {
            node.as_ref().unwrap().val
        } else {
            -1
        }
    }

    fn add_at_head(&mut self, val: i32) {
        let new_node = Node {
            val,
            next: self.head.take(),
        };

        self.head.replace(Box::new(new_node));
    }

    fn add_at_tail(&mut self, val: i32) {
        let mut node = &mut self.head;
        while node.is_some() {
            node = &mut node.as_mut().unwrap().next;
        }

        node.replace(Box::new(Node { val, next: None }));
    }

    #[allow(unused)]
    fn add_at_index(&mut self, index: i32, val: i32) {
        let mut node = &mut self.head;
        let mut count = index;

        while node.is_some() {
            if count <= 0 {
                break;
            }
            node = &mut node.as_mut().unwrap().next;
            count -= 1;
        }

        if count == 0 {
            let old_node = node.replace(Box::new(Node { val, next: None }));

            node.as_mut().unwrap().next = old_node;
        }
    }

    fn delete_at_index(&mut self, index: i32) {
        let mut node = &mut self.head;
        let mut count = index;
        while node.is_some() {
            if count <= 0 {
                break;
            }
            node = &mut node.as_mut().unwrap().next;
            count -= 1;
        }

        if count == 0 && node.is_some() {
            let mut next = node.as_mut().unwrap().next.take();
            std::mem::swap(&mut next, node);
        }
    }
}

pub mod tests {
    use super::MyLinkedList;

    #[allow(unused)]
    pub fn test() {
        let mut obj = MyLinkedList::new();
        let index = 0;
        let val = 2;

        obj.add_at_head(2);
        obj.delete_at_index(1);
        obj.add_at_head(2);
        obj.add_at_head(7);
        obj.add_at_head(3);
        obj.add_at_head(2);
        obj.add_at_head(5);
        obj.add_at_tail(5);
        obj.get(5);
        obj.delete_at_index(6);
        obj.delete_at_index(4);

        let ret_1: i32 = obj.get(index);
    }
}
