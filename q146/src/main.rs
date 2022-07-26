use std::collections::HashMap;
use std::ptr;

fn main() {}

struct Node {
    key: i32,
    val: i32,
    prev: *mut Node,
    next: *mut Node,
}

impl Node {
    fn new(key: i32, val: i32) -> Self {
        Self {
            key,
            val,
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
        }
    }
}

struct DLinkedList {
    head: Box<Node>,
    tail: Box<Node>,
}

impl DLinkedList {
    // head and left are just dummy nodes
    fn new() -> Self {
        let mut head = Box::new(Node::new(-1, -1));
        let mut tail = Box::new(Node::new(-2, -2));
        head.next = &mut *tail;
        tail.prev = &mut *head;
        Self { head, tail }
    }

    fn insert(&mut self, node: *mut Node) {
        println!("inserting");
        unsafe {
            let next = (*self.head).next;
            (*node).next = &mut *next;
            (*node).prev = &mut *self.head;
            (*self.head).next = &mut *node;
            (*next).prev = &mut *node;
        }
    }

    fn remove(&mut self, node: *mut Node) {
        println!("removing");
        unsafe {
            let prev = (*node).prev;
            let next = (*node).next;
            (*prev).next = next;
            (*next).prev = prev;
        }
    }

    fn pop(&mut self) -> i32 {
        println!("popping");
        unsafe {
            // left <-> mid(to be removed) <-> right(tail)
            let mid = (*self.tail).prev;
            let left = (*mid).prev;
            (*left).next = &mut *self.tail;
            (*self.tail).prev = left;
            // left <-> right(tail)
            (*mid).key
        }
    }
}

struct LRUCache {
    capacity: i32,
    cache: HashMap<i32, Box<Node>>,
    list: DLinkedList,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            capacity,
            cache: HashMap::new(),
            list: DLinkedList::new(),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        match self.cache.get_mut(&key) {
            None => return -1,
            Some(n) => {
                self.list.remove(&mut **n);
                self.list.insert(&mut **n);
                n.val
            }
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        let mut node = Box::new(Node::new(key, value));
        let node_addr = &mut *node;

        match self.cache.get_mut(&key) {
            None => {
                if self.cache.len() as i32 >= self.capacity {
                    self.cache.remove(&self.list.pop());
                }
            }
            Some(n) => {
                self.list.remove(&mut **n);
            }
        }
        self.list.insert(node_addr);
        self.cache.insert(key, node);
    }
}

#[test]
fn test1() {
    let mut lru = LRUCache::new(10);
    for i in 1..10 {
        lru.put(i, 1);
    }
    for i in 1..10 {
        lru.get(i);
    }
}
