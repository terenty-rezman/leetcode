use std::cell::RefCell;
use std::rc::Rc;
struct Solution;

use crate::binary_tree::TreeNode;

impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_sum = i32::MIN;
        Self::postorder(&root, &mut max_sum);
        max_sum
    }

    fn postorder(node: &Option<Rc<RefCell<TreeNode>>>, max_sum: &mut i32) -> i32 {
        use std::cmp::max;
        if node.is_none() {
            return 0;
        }

        let n = node.as_ref().unwrap();
        let val = n.borrow().val;

        let left_sum = Self::postorder(&n.borrow().left, max_sum);
        let right_sum = Self::postorder(&n.borrow().right, max_sum);

        let local_max_sum = val
            .max(val + left_sum)
            .max(val + right_sum)
            .max(val + right_sum + left_sum);
        *max_sum = max(*max_sum, local_max_sum);

        val.max(val + left_sum).max(val + right_sum)
    }
}

pub mod tests {
    use super::Solution;
    use crate::binary_tree::from_bfs_array;

    pub fn test() {
        let tree = from_bfs_array(&[Some(2), Some(-1), Some(-2)]);
        let r = Solution::max_path_sum(tree);
        dbg!(r);
    }
}
