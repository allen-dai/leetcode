use std::collections::VecDeque;

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

pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    if lists.len() == 0 {
        return None;
    }

    let mut lists = VecDeque::from(lists);

    while lists.len() > 1 {
        let left = lists.pop_back().unwrap_or(None);
        let right = lists.pop_back().unwrap_or(None);
        lists.push_front(merge(left, right));
    }

    lists.pop_back().unwrap()
}

pub fn merge(
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
            // list1 = curr
            mem::swap(res_ref, &mut list1);
        } else {
            // 'curr' <- list1
            mem::swap(res_ref, &mut list2);
            // curr = curr.next
            res_ref = &mut res_ref.as_mut().unwrap().next;
            // list2 = curr
            mem::swap(res_ref, &mut list2);
        }
    }
}
