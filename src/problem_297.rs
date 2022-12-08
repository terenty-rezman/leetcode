use std::cell::RefCell;
use std::rc::Rc;

use crate::binary_tree::TreeNode;

struct Codec {}

impl Codec {
    fn new() -> Self {
        Codec {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut result = String::new();
        Codec::preorder(&root, &mut result);
        result
    }

    fn preorder(node: &Option<Rc<RefCell<TreeNode>>>, result: &mut String) {
        if let Some(node) = node.as_ref() {
            let node = node.borrow();
            result.push_str((node.val.to_string() + " ").as_str());

            Codec::preorder(&node.left, result);
            Codec::preorder(&node.right, result);
        } else {
            result.push_str(("#".to_string() + " ").as_str());
        }
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let items: Vec<Option<i32>> = data.split(" ").map(|s| s.parse::<i32>().ok()).collect();
        let mut slice = &items[..];
        Codec::build_from_preorder(&mut slice)
    }

    fn build_from_preorder(items: &mut &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
        if items.is_empty() {
            None
        } else {
            let val = items.first().unwrap();
            *items = &items[1..];

            if let Some(val) = val {
                let node = TreeNode {
                    val: *val,
                    left: Codec::build_from_preorder(items),
                    right: Codec::build_from_preorder(items),
                };

                Some(Rc::new(RefCell::new(node)))
            } else {
                None
            }
        }
    }
}

pub mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::Codec;
    use crate::binary_tree::{self, TreeNode};

    pub fn test() {
        let root = binary_tree::from_level_order(&[Some(1), Some(2), Some(3)]);

        let obj = Codec::new();
        let data: String = obj.serialize(root);
        let ans: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);
        dbg!(binary_tree::to_level_order(&ans));
        // dbg!(ans);
    }
}
