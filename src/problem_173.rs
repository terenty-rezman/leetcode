use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::binary_tree::TreeNode;

struct BSTIterator {
    q: VecDeque<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        Self {
            q: VecDeque::default(),
            right: root,
        }
    }

    fn next(&mut self) -> i32 {
        let mut is_right = false;
        let mut node = {
            if self.right.is_some() {
                is_right = true;
                self.right.take().unwrap()
            } else {
                self.q.pop_back().unwrap()
            }
        };

        if is_right {
            while node.borrow().left.is_some() {
                let b = node.borrow();
                self.q.push_back(node.clone());
                let left = b.left.as_ref().unwrap().clone();
                drop(b);
                node = left;
            }
        }

        self.right = node.borrow().right.clone();

        let x = node.borrow().val;
        x
    }

    fn has_next(&self) -> bool {
        self.right.is_some() || !self.q.is_empty()
    }
}

pub mod tests {
    use super::BSTIterator;
    use crate::binary_tree;

    pub fn test() {
        let root = binary_tree::from_level_order(&[Some(1), Some(2), Some(3)]);
        let mut iter = BSTIterator::new(root);

        while iter.has_next() {
            let node = iter.next();
            dbg!(node);
        }
    }
}
