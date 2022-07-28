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

use std::cell::{RefCell, RefMut};
use std::rc::Rc;
struct Solution;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            1 + i32::max(
                Self::max_depth(node.borrow().left.clone()),
                Self::max_depth(node.borrow().right.clone()),
            )
        } else {
            0
        }
    }
}
