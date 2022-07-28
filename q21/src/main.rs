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

pub fn reverse_list(
    mut list1: Option<Box<ListNode>>,
    mut list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    use std::mem;
    let mut res = None;
    let mut res_ref = &mut res;

    loop {
        if list1.is_none() {
            mem::swap(&mut list2, &mut res_ref);
            return res;
        }

        if list2.is_none() {
            mem::swap(&mut list1, &mut res_ref);
            return res;
        }

        if list1.as_ref().unwrap().val < list2.as_ref().unwrap().val {
            // 'curr' <- list1
            mem::swap(res_ref, &mut list1);
            // curr = curr.next
            res_ref = &mut res_ref.as_mut().unwrap().next;
            // list1 = next
            mem::swap(res_ref, &mut list1);
        } else {
            // 'curr' <- list1
            mem::swap(res_ref, &mut list2);
            // curr = curr.next
            res_ref = &mut res_ref.as_mut().unwrap().next;
            // list1 = next
            mem::swap(res_ref, &mut list2);
        }
    }
}

/* -- For neetcode 
pub fn reverse_list(
    mut list1: Option<Box<ListNode>>,
    mut list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode::new(-1));
    let mut tail = &mut dummy;

    while list1.is_some() && list2.is_some() {
        if list1.as_ref().unwrap().val < list2.as_ref().unwrap().val {
            let nxt = list1.as_mut().unwrap().next.take();
            tail.next = list1;
            list1 = nxt;
        } else {
            let nxt = list2.as_mut().unwrap().next.take();
            tail.next = list2;
            list2 = nxt;
        }
        tail = tail.next.as_mut().unwrap();
    }

    if list1.is_some() {
        tail.next = list1;
    } else if list2.is_some() {
        tail.next = list2;
    }

    dummy.next
} */
