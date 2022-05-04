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

pub fn from_bfs_array(array: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
    let mut values = VecDeque::from(array.to_vec());
    let mut root: Option<Rc<RefCell<TreeNode>>> = None;

    if !array.is_empty() {
      let mut to_process = VecDeque::new();
      let val = values.pop_front().unwrap().unwrap();

      root.replace(Rc::new(RefCell::new(TreeNode::new(val))));
      to_process.push_back(root.as_ref().unwrap().clone());
      
      while !values.is_empty() {
        let node = to_process.pop_front().unwrap();

        if let Some(val ) = values.pop_front().unwrap() {
          node.borrow_mut().left.replace(Rc::new(RefCell::new(TreeNode::new(val))));
          to_process.push_back(node.borrow().left.clone().unwrap());
        }

        if values.is_empty() {
          break;
        }

        if let Some(val ) = values.pop_front().unwrap() {
          node.borrow_mut().right.replace(Rc::new(RefCell::new(TreeNode::new(val))));
          to_process.push_back(node.borrow().right.clone().unwrap());
        }
      }
    }

    root    
}

pub fn print_level_order(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
  let mut result = vec![];
  let mut to_process = VecDeque::new();
  to_process.push_back(root.clone());

  while !to_process.is_empty() {
    let node = to_process.pop_front().unwrap();
    if let Some(node) = node {
      result.push(Some(node.borrow().val));
      to_process.push_back(node.borrow().left.clone());
      to_process.push_back(node.borrow().right.clone());
    } else {
      result.push(None);
    }
  }

  result
}