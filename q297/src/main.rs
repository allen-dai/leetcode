#![no_main]
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

struct Codec {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Self {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let lo = Self::level_order(root);
        let mut ans: String = String::from("[");

        for level in lo {
            for node in level {
                let content = node + ",";
                ans += &content;
            }
        }
        if ans.len() > 1 {
            ans.pop();
        }
        ans += "]";
        println!("{}", ans);
        ans
    }

    fn deserialize(&self, mut data: String) -> Option<Rc<RefCell<TreeNode>>> {
        use std::collections::VecDeque;
        if data.len() == 2 {
            return None;
        }
        // removing '[' ']'
        data.remove(0);
        data.pop();

        let content: Vec<&str> = data.split(",").collect();
        let nodes: VecDeque<Option<Rc<RefCell<TreeNode>>>> = content
            .iter()
            .map(|c| {
                if c == &"null" {
                    return None;
                }

                Some(Rc::new(RefCell::new(TreeNode::new(
                    c.parse::<i32>().unwrap(),
                ))))
            })
            .collect();

        let mut kids = nodes.clone();
        let root = kids.pop_front().unwrap();

        for node in nodes {
            if node.is_some() {
                if !kids.is_empty() { node.as_ref().unwrap().borrow_mut().left = kids.pop_front().unwrap() }
                if !kids.is_empty() { node.as_ref().unwrap().borrow_mut().right = kids.pop_front().unwrap() }
            }
        }

        root
    }

    fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<String>> {
        fn travelsal(
            root: &Option<Rc<RefCell<TreeNode>>>,
            nodes: &mut Vec<Vec<String>>,
            mut depth: usize,
        ) {
            depth += 1;
            if nodes.len() < depth {
                nodes.push(vec![]);
            }

            if root.is_none() {
                if depth > 1 {
                    nodes[depth - 1].push("null".to_owned());
                }
                return;
            }

            nodes[depth - 1].push(root.as_ref().unwrap().borrow().val.to_string());
            travelsal(&root.as_ref().unwrap().borrow().left, nodes, depth);
            travelsal(&root.as_ref().unwrap().borrow().right, nodes, depth);
        }

        let mut nodes = Vec::new();
        travelsal(&root, &mut nodes, 0);
        if nodes.len() > 1 {
            nodes.pop();
        }
        nodes
    }
}

/*
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let data: String = obj.serialize(strs);
 * let ans: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);
 */
