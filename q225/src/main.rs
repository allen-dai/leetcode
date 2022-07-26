struct Stack {
    stack: Vec<i32>,
}

impl Stack {
    fn new() -> Self {
        Stack { stack: Vec::new() }
    }

    fn push(&mut self, x: i32) {
        self.stack.push(x);
    }

    fn pop(&mut self) -> i32{
        return self.stack.pop().unwrap();
    }

    fn top(&mut self) -> Option<i32>{
        return Some(self.stack[0]);
    }

    fn empty(&self) -> bool {
        return self.stack.len() == 0;
    }
}

fn main() {
}
