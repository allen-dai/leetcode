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
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: &Rc<RefCell<TreeNode>>, ans: &mut i32) -> i32 {
            let curr_val = root.borrow().val;
            let mut left_sum = curr_val;
            let mut right_sum = curr_val;
            if let Some(node) = root.borrow().left.as_ref() {
                let left_path_sum = dfs(&node, ans);
                if left_sum + left_path_sum > left_sum {
                    left_sum += left_path_sum;
                }
            }

            if let Some(node) = root.borrow().right.as_ref() {
                let right_path_sum = dfs(node, ans);
                if left_sum + right_path_sum > left_sum {
                    right_sum += right_path_sum;
                }
            }

            *ans = (*ans)
                .max(left_sum)
                .max(right_sum)
                .max(left_sum + right_sum - curr_val);

            left_sum.max(right_sum)
        }

        let mut ans = 0;
        if root.is_some() {
            ans = root.as_ref().unwrap().borrow().val;
            dfs(&root.unwrap(), &mut ans);
        }

        ans
    }
}

fn main() {
    println!("Hello, world!");
}
