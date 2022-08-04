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
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, ans: *mut Vec<Vec<i32>>, mut d: usize) {
            if root.is_none() {
                return;
            }
            d += 1;
            unsafe {
                if (*ans).len() < d {
                    (*ans).push(vec![]);
                }

                (*ans)[d - 1].push(root.as_ref().unwrap().borrow_mut().val);
                dfs(root.as_ref().unwrap().borrow_mut().left.take(), ans, d);
                dfs(root.as_ref().unwrap().borrow_mut().right.take(), ans, d);
            }
        }

        let mut ans = Vec::new();
        dfs(root, &mut ans, 0);
        ans
    }
}

fn main() {
    println!("Hello, world!");
}
