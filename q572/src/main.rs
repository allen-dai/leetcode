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
        fn dfs(r: &Option<Rc<RefCell<TreeNode>>>, s: &Option<Rc<RefCell<TreeNode>>>) -> bool {
            if s.is_none() {
                return true;
            }
            if r.is_none() {
                return false;
            }
            if same_tree(r, s) {
                return true;
            }

            dfs(&r.as_ref().unwrap().borrow().left, s)
                || dfs(&r.as_ref().unwrap().borrow().right, s)
        }

        fn same_tree(r: &Option<Rc<RefCell<TreeNode>>>, s: &Option<Rc<RefCell<TreeNode>>>) -> bool {
            match (r, s) {
                (Some(r_n), Some(s_n)) => {
                    if r_n.borrow().val == s_n.borrow().val {
                        same_tree(&r_n.borrow().left, &s_n.borrow().left)
                            && same_tree(&r_n.borrow().right, &s_n.borrow().right)
                    } else {
                        return false;
                    }
                }

                (None, None) => return true,

                _ => return false,
            }
        }

        dfs(&root, &sub_root)
    }
}
