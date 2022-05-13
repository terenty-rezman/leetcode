use crate::binary_tree::{self, TreeNode};

struct Solution;

use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::Ordering;

impl Solution {
    pub fn insert_into_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        let mut node = root.clone();
        while let Some(nd) = node.clone() {
            match val.cmp(&nd.borrow().val) {
                Ordering::Less => {
                    if nd.borrow().left.is_some() {
                        node = nd.borrow().left.clone();
                    } else {
                        nd.borrow_mut().left.replace(Rc::new(RefCell::new(TreeNode::new(val))));
                    }
                }
                Ordering::Greater => {
                    if nd.borrow().right.is_some() {
                        node = nd.borrow().right.clone();
                    } else {
                        nd.borrow_mut().right.replace(Rc::new(RefCell::new(TreeNode::new(val))));
                    }
                }
                _ => unreachable!()
            }
        }
        root
    }
}

pub mod tests {
    use crate::binary_tree::{self, TreeNode};
    use super::*;

    pub fn test() {
        let root = binary_tree::from_bfs_array(&[Some(1), Some(2), Some(3)]);

        let root = Solution::insert_into_bst(root, 4);
        dbg!(binary_tree::print_level_order(&root));
    }
}