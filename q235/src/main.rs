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

use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;
struct Solution;
impl Solution {
    pub fn lowest_common_ancestor(
        mut root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match (p, q) {
            (Some(p), Some(q)) => {
                let (p, q) = (p.try_borrow().unwrap(), q.try_borrow().unwrap());
                while let Some(n) = root.clone() {
                    let n = n.try_borrow().unwrap();
                    if i32::max(p.val, q.val) < n.val {
                        root = n.left.clone();
                    } else if i32::min(p.val, q.val) > n.val {
                        root = n.right.clone();
                    } else {
                        return root;
                    }
                }
                None
            }
            _ => None,
        }
    }
}

fn main() {}
