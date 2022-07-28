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
    pub fn dfs(root: Option<Rc<RefCell<TreeNode>>>, d: *mut i32) -> i32 {
        if let Some(node) = root {
            let left = Self::dfs(node.borrow().left.clone(), d);
            let right = Self::dfs(node.borrow().right.clone(), d);
            unsafe {
                *d = i32::max(*d, left + right);
            }

            1 + i32::max(left, right)
        } else {
            0
        }
    }
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res: i32 = 0;
        Self::dfs(root, &mut res);

        res
    }
}
