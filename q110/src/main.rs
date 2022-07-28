fn main() {}

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
struct Solution;

impl Solution {
    pub fn dfs(root: Option<Rc<RefCell<TreeNode>>>, depth: *mut i32, diff: *mut i32) -> bool {
        if let Some(node) = root {
            Self::dfs(node.borrow().left.clone(), d) == Self::dfs(node.borrow().right.clone(), d)
        } else {
            unsafe {
                if *diff == 0 && *depth == 0 {
                    return true;
                }
                *diff <= 1
            }
        }
    }
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut depth = 0;
        let mut diff = 0;
        Self::dfs(root, &mut depth, &mut diff)
    }
}
