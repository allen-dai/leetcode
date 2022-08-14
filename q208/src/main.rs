#![no_main]
#![allow(dead_code)]

use std::collections::HashMap;

struct Trie {
    root: Node,
}

#[derive(Default)]
struct Node {
    children: HashMap<char, Node>,
    eow: bool,
}
/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    fn new() -> Self {
        Self {
            root: Node::default(),
        }
    }

    fn insert(&mut self, word: String) {
        let mut curr = &mut self.root;
        for c in word.chars() {
            if !curr.children.contains_key(&c) {
                curr.children.insert(c, Node::default());
            }
            curr = curr.children.get_mut(&c).unwrap();
        }
        curr.eow = true;
    }

    fn search(&mut self, word: String) -> bool {
        let mut curr = &mut self.root;
        for c in word.chars() {
            if !curr.children.contains_key(&c) {
                return false;
            }
            curr = curr.children.get_mut(&c).unwrap();
        }
        return curr.eow;
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut curr = &self.root;

        for c in prefix.chars() {
            if !curr.children.contains_key(&c) {
                return false;
            }
            curr = curr.children.get(&c).unwrap();
        }

        true
    }
}
