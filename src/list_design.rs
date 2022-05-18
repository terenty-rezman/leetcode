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
        let mut node: &Node = self.head.as_ref().unwrap();

        while count > 0 {
            node = node.next.as_ref().unwrap();
            count -= 1;
        }

        node.val
    }

    fn add_at_head(&mut self, val: i32) {
        let new_node = Node {
            val: val,
            next: self.head.take(),
        };

        self.head.replace(Box::new(new_node));
    }

    fn add_at_tail(&mut self, val: i32) {
        let mut node = &mut self.head;
        while node.is_some() {
            node = &mut node.as_mut().unwrap().next;
        }

        node.replace(Box::new(Node {
            val: val,
            next: None,
        }));
    }

    fn add_at_index(&mut self, index: i32, val: i32) {
        let mut node = &mut self.head; 
        let mut count = index;
        while count > 0 {
            node = &mut node.as_mut().unwrap().next;
            count -= 1;
        }

        let old_node = node.replace(
            Box::new(Node {val: val, next: None})
        );

        node.as_mut().unwrap().next = old_node;
    }

    fn delete_at_index(&mut self, index: i32) {
        let mut node = &mut self.head;
        let mut count = index;
        while count > 0 {
            node = &mut node.as_mut().unwrap().next;
            count -= 1;
        }
        
        let mut next = node.as_mut().unwrap().next.take();
        std::mem::swap(&mut next, node);
    }
}

pub mod tests {
    use super::MyLinkedList;

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
