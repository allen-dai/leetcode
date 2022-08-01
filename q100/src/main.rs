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
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, v: *mut Vec<i32>) {
            if let Some(node) = root {
                unsafe {
                    (*v).push(node.borrow().val);
                }
                dfs(node.borrow_mut().left.take(), v);
                dfs(node.borrow_mut().right.take(), v);
            } else {
                unsafe {
                    (*v).push(-1);
                }
                return;
            }
        }

        let mut p_v = Vec::new();
        let mut q_v = Vec::new();

        dfs(p, &mut p_v);
        dfs(q, &mut q_v);

        p_v.eq(&q_v)
    }
}
