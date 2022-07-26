use std::collections::VecDeque;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
fn main() {}

pub fn recorder_lsit(mut head: &mut Option<Box<ListNode>>) {
    let mut nodes = VecDeque::new();
    let mut node = head.as_mut().unwrap().next.take();
    let mut count = 1;
    while node.is_some() {
        let tmp = node.as_mut().unwrap().next.take();
        nodes.push_back(node);
        node = tmp;
        count += 1;
    }

    println!("{:?}", nodes);
}
