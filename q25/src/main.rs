fn main() {}

#[derive(PartialEq, Eq, Debug)]
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

pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut nodes = Vec::new();
    while let Some(mut n) = head {
        // head = n.next, n.next -> None
        head = n.next.take();
        // [ nodes { val: *, next: None } ... ]
        nodes.push(Some(n));
    }

    let mut tmp = Some(Box::new(ListNode::new(69)));
    let mut curr = &mut tmp;

    for chunk in nodes.chunks_mut(k as usize) {
        let mut nodes: Vec<&mut Option<Box<ListNode>>> = if chunk.len() == k as usize {
            chunk.iter_mut().rev().collect()
        } else {
            chunk.iter_mut().collect()
        };

        for node in nodes.iter_mut(){
            curr.as_mut().unwrap().next = node.take();
            curr = &mut curr.as_mut().unwrap().next;
        }
    }

    tmp.unwrap().next
}
