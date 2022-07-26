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
