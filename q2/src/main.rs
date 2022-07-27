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

pub fn add_two_numbers(
    mut l1: Option<Box<ListNode>>,
    mut l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    use std::mem;

    let mut carry = 0;
    let mut tmp = Some(Box::new(ListNode::new(0)));
    let mut curr = &mut tmp;

        println!("{}\n", 12 % 10);
    while l1.is_some() || l2.is_some() {
        let l1_n = if l1.is_some() {
            l1.as_ref().unwrap().val
        } else {
            0
        };
        let l2_n = if l2.is_some() {
            l2.as_ref().unwrap().val
        } else {
            0
        };

        let n = l1_n + l2_n;
        let mut node = Some(Box::new(ListNode::new( (n + carry) % 10)));
        carry = 0;
        if n >= 10 || carry + n >= 10 {
            carry = 1;
        }
        if l1.is_some() {
            l1 = l1.as_mut().unwrap().next.take();
        }

        if l2.is_some() {
            l2 = l2.as_mut().unwrap().next.take();
        }

        mem::swap(&mut curr.as_mut().unwrap().next, &mut node);
        curr = &mut curr.as_mut().unwrap().next;
    }

    tmp.as_mut().unwrap().next.take()
}
fn test_1() {
    use std::mem;
    let mut l1 = Some(Box::new(ListNode::new(2)));
    let mut curr = &mut l1;
    let l1_nums = vec![1, 2, 3, 9, 9, 9, 9, 9];
    for i in l1_nums {
        let mut node = Some(Box::new(ListNode::new(i)));
        mem::swap(&mut curr.as_mut().unwrap().next, &mut node);
        curr = &mut curr.as_mut().unwrap().next;
    }

    let mut l2 = Some(Box::new(ListNode::new(5)));
    let mut curr = &mut l2;
    let l2_nums = vec![7, 9, 8, 7, 9, 9];
    for i in l2_nums {
        let mut node = Some(Box::new(ListNode::new(i)));
        mem::swap(&mut curr.as_mut().unwrap().next, &mut node);
        curr = &mut curr.as_mut().unwrap().next;
    }

    /* let mut curr = &head;
    while let Some(n) = curr {
        curr = &n.next;
        println!("{}", n.val);
    } */

    println!("\n\n");
    let head = add_two_numbers(l1, l2);
    let mut curr = &head;
    while let Some(n) = curr {
        curr = &n.next;
        println!("{}", n.val);
    }
}
