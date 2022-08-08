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
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn build(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if preorder.is_empty() {
                return None;
            }

            let mut root = Some(Rc::new(RefCell::new(TreeNode::new(preorder[0]))));
            let mid_idx = inorder.iter().position(|&x| x == preorder[0]).unwrap();
            root.as_mut().unwrap().borrow_mut().left =
                build(&preorder[1..mid_idx + 1], &inorder[..mid_idx]);
            root.as_mut().unwrap().borrow_mut().right =
                build(&preorder[mid_idx + 1..], &inorder[mid_idx + 1..]);
            root
        }

        build(&preorder, &inorder)
    }
}
fn main() {}
