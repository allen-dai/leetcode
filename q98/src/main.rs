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
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut res: bool = true;
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, res: *mut bool, mn: i32, mx: i32) {
            if root.is_none() {
                return;
            }

            let n = root.unwrap();
            let l = n.borrow_mut().left.take();
            let r = n.borrow_mut().right.take();

            if l.is_some() && l.as_ref().unwrap().borrow_mut().val >= n.borrow_mut().val {
                unsafe {
                    (*res) = false;
                }
            }
            if r.is_some() && r.as_ref().unwrap().borrow_mut().val <= n.borrow_mut().val {
                unsafe {
                    (*res) = false;
                }
            }
            dfs(l, res, mn, mx);
            dfs(r, res, mn, mx);
        }

        dfs(root, &mut res, i32::MAX, i32::MIN);

        res
    }
}

fn main() {
    println!("Hello, world!");
}
