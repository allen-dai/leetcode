#[derive(Default)]
struct Node {
    eow: bool,
    children: [Option<Box<Node>>; 26],
}

struct WordDictionary {
    root: Option<Box<Node>>,
}

impl WordDictionary {
    fn new() -> Self {
        Self {
            root: Some(Box::new(Node::default())),
        }
    }

    fn add_word(&mut self, word: String) {
        let mut curr = &mut self.root;
        for c in word.chars() {
            let idx = (c as u8 - b'a') as usize;
            if let Some(n) = curr {
                if n.children[idx].is_none() {
                    n.children[idx] = Some(Box::new(Node::default()));
                }
                curr = &mut n.children[idx];
            }
        }
        curr.as_mut().unwrap().eow = true;
    }

    fn search(&self, word: String) -> bool {
        fn dfs(root: &Option<Box<Node>>, word: &str) -> bool {
            if root.is_none() {
                return false;
            }

            if word.is_empty() {
                return root.as_ref().unwrap().eow;
            }

            if word.as_bytes()[0] == '.' as u8 {
                return (0..26)
                    .map(|ch| dfs(&root.as_ref().unwrap().children[ch], &word[1..]))
                    .any(|b| b);
            }

            let idx = (word.as_bytes()[0] - b'a') as usize;
            if root.as_ref().unwrap().children[idx].is_none() {
                return false;
            }

            dfs(&root.as_ref().unwrap().children[idx], &word[1..])
        }

        dfs(&self.root, &word)
    }
}

fn main() {
    let mut t = WordDictionary::new();
    t.add_word("at".to_string());
    t.add_word("and".to_string());
    t.add_word("an".to_string());
    t.add_word("add".to_string());
    let res = t.search("a".to_string());

    println!("{}", res);
}
