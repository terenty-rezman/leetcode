use core::borrow;
use std::cell::{RefCell, RefMut};
use std::rc::Rc;

use crate::binary_tree::TreeNode;
struct Solution;

impl Solution {
    pub fn invert_tree(mut root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn invert(node: &mut Option<Rc<RefCell<TreeNode>>>) {
            if node.is_none() {
                return;
            }

            let n = node.as_mut().unwrap();
            invert(&mut n.borrow_mut().left);
            invert(&mut n.borrow_mut().right);

            let mut borrow = n.borrow_mut();

            let temp = borrow.left.take();
            borrow.left = borrow.right.take();
            borrow.right = temp;
        }

        invert(&mut root);
        root
    }
}

pub mod tests {
    use super::Solution;
    use crate::binary_tree::{from_level_order, to_level_order};

    pub fn test() {
        let r = Solution::invert_tree(from_level_order(&[
            Some(1),
            Some(2),
            Some(3)
        ]));
        let r  = to_level_order(&r);
        dbg!(r);
    }
}
