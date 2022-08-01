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
    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if root.is_none() {
            return false;
        }
        fn dfs(t: &Option<Rc<RefCell<TreeNode>>>, s: &Option<Rc<RefCell<TreeNode>>>) -> bool {
            if t.is_none() {
                return true;
            }
            if s.is_none() {
                return false;
            }
            if dfs(t, s) {
                return true;
            }

            dfs(t, &s.as_ref().unwrap().borrow().left)
                || dfs(t, &s.as_ref().unwrap().borrow().right)
        }

        if root.is_none() && sub_root.is_none() {
            return true;
        }

        if root.is_none() || sub_root.is_none() {
            return false;
        }

        if root.as_ref().unwrap().borrow().val == sub_root.as_ref().unwrap().borrow().val {
            return dfs(
                &root.as_ref().unwrap().borrow().left,
                &sub_root.as_ref().unwrap().borrow().left,
            ) && dfs(
                &root.as_ref().unwrap().borrow().right,
                &sub_root.as_ref().unwrap().borrow().right,
            );
        };

        false
    }
}
