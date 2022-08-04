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
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res: i32 = 0;

        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, mut mx: i32, good: *mut i32) {
            if root.is_none() {
                return;
            }
            let r_v = root.as_ref().unwrap().borrow_mut().val;
            if r_v > mx {
                unsafe {
                    (*good) += 1;
                }
                mx = r_v
            }

            dfs(root.as_ref().unwrap().borrow_mut().left.take(), mx, good);
            dfs(root.as_ref().unwrap().borrow_mut().right.take(), mx, good);
        }

        dfs(root, i32::MIN, &mut res);

        res
    }
}

fn main() {
    println!("Hello, world!");
}
