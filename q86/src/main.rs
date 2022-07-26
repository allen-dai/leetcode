fn main() {}

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

pub fn partition(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
    let mut nodes = Vec::new();

    while let Some(mut node) = head {
        head = node.next.take();
        nodes.push(node);
    }

    let mut idx = Vec::new();
    let mut l = Vec::new();
    for (i, n) in nodes.iter().enumerate() {
        if n.val < x {
            idx.push(i);
        } else {
            l.push(i);
        }
    }

    idx.extend(l);

    if idx.len() < 1 {
        return None;
    }

    let mut head = Some(nodes[idx[0]].clone());
    let mut curr = &mut head;
    for i in idx.iter().skip(1) {
        curr.as_mut().unwrap().next = Some(nodes[*i].clone());
        curr = &mut curr.as_mut().unwrap().next;
    }

    head
}
