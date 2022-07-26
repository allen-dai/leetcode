fn main() {
    println!("Hello, world!");
}

struct MinStack {
    stack: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    fn new() -> Self {
        MinStack { stack: Vec::new() }
    }

    fn push(&mut self, val: i32) {
        if self.stack.is_empty() {
            self.stack.push(vec![val, val]);
            return;
        }

        let min = std::cmp::min(self.stack.last().unwrap()[1], val);

        self.stack.push(vec![val, min]);
    }

    fn pop(&mut self) {
        self.stack.pop();
    }

    fn top(&self) -> i32 {
        return self.stack.last().unwrap()[0];
    }

    fn get_min(&self) -> i32 {
        return return self.stack[self.stack.len() - 1][1];
    }
}
