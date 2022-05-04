use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

pub fn from_bfs_array(array: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    let mut root: Option<Rc<RefCell<TreeNode>>> = None;
    let mut values = VecDeque::from(array.to_vec());
    let mut queue_to_process = VecDeque::from([&mut root]);

    while !queue_to_process.is_empty() && !values.is_empty() {
        let node = queue_to_process.pop_front().unwrap();
        let val = values.pop_front().unwrap();

        let new_node = Rc::new(RefCell::new(TreeNode::new(val)));
        node.replace(new_node);
    }

    root    
}
