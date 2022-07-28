fn main() {
    println!("Hello, world!");
}

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
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;

pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(node) = root {
        let l = node.borrow().left.clone();
        let r = node.borrow().right.clone();
        node.borrow_mut().left = self::invert_tree(r);
        node.borrow_mut().right = self::invert_tree(l);

        Some(node)
    } else {
        None
    }
}
