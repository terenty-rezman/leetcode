use std::cell::RefCell;
use std::rc::Rc;

use crate::binary_tree::TreeNode;
use crate::my_list::print_list;

struct Solution;
type Node = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn lowest_common_ancestor(root: Node, p: Node, q: Node) -> Option<Rc<RefCell<TreeNode>>> {
        type Node = Option<Rc<RefCell<TreeNode>>>;

        fn postorder(node: Node, p: &Node, q: &Node, result: &mut Node) -> [bool; 2] {
            if node.is_none() {
                return [false, false];
            }

            let n = node.unwrap();
            let [mut p_found, mut q_found] = postorder(n.borrow_mut().left.take(), p, q, result);
            let [p_found_2, q_found_2] = postorder(n.borrow_mut().right.take(), p, q, result);
            p_found |= p_found_2;
            q_found |= q_found_2;

            dbg!(n.borrow().val);

            p_found |= n.borrow().val == p.as_ref().unwrap().borrow().val;
            q_found |= n.borrow().val == q.as_ref().unwrap().borrow().val;

            if p_found && q_found && result.is_none() {
                result.replace(Rc::clone(&n));
            }

            [p_found, q_found]
        }

        let mut result = None;
        postorder(root, &p, &q, &mut result);
        result
    }
}

pub mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::Solution;
    use crate::binary_tree::{from_level_order, TreeNode};

    pub fn test() {
        let input = from_level_order(&[
            Some(6),
            Some(2),
            Some(8),
            Some(0),
            Some(4),
            Some(7),
            Some(9),
            None,
            None,
            Some(3),
            Some(5),
        ]);
        let r = Solution::lowest_common_ancestor(
            input,
            Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            Some(Rc::new(RefCell::new(TreeNode::new(4)))),
        );
        dbg!(r);
    }
}
