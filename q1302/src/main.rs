use std::{borrow::Borrow, cell::RefCell, rc::Rc};

#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(val: i32) -> TreeNode {
        return TreeNode {
            val: val,
            left: None,
            right: None,
        };
    }
}

fn main() {
    let mut t = TreeNode::new();
    let t1 = &t;
    let t2 = &t;
    println!("{:?} {:?}", t1, t2);
}

fn ans(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    return 0;
}
