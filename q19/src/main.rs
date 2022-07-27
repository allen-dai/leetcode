fn main() {
    test_1();
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
//
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    use std::mem;
    if head.is_none() || (head.as_mut().unwrap().next.is_none() && n == 1) {
        return None;
    }

    let mut nodes = Vec::new();
    let mut curr = head;

    while let Some(mut n) = curr {
        curr = n.next.take();
        nodes.push(Some(n));
    }

    nodes.remove(nodes.len() - n as usize);

    if nodes.is_empty() {
        return None;
    }

    nodes.reverse();

    let mut head = nodes.pop().unwrap();
    let mut curr = &mut head;

    while !nodes.is_empty() {
        let mut node = nodes.pop().unwrap();
        mem::swap(&mut curr.as_mut().unwrap().next, &mut node);
        curr = &mut curr.as_mut().unwrap().next;
    }

    head
}

fn test_1() {
    use std::mem;
    let mut head = Some(Box::new(ListNode::new(1)));
    let mut curr = &mut head;
    for i in 2..2 {
        let mut node = Some(Box::new(ListNode::new(i)));
        mem::swap(&mut curr.as_mut().unwrap().next, &mut node);
        curr = &mut curr.as_mut().unwrap().next;
    }

    let mut curr = &head;
    while let Some(n) = curr {
        curr = &n.next;
        println!("{}", n.val);
    }

    println!("\n\n");
    let head = remove_nth_from_end(head, 1);
    let mut curr = &head;
    while let Some(n) = curr {
        curr = &n.next;
        println!("{}", n.val);
    }
}
