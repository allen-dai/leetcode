fn main() {
    println!("{:?}", ans(String::from("(()(()))")));
}

fn ans(s: String) -> i32 {
    let mut stack = vec![0];
    for c in s.chars() {
        println!("{:?}", stack);
        match c {
            '(' => stack.push(0),
            _ => {
                let p = stack.pop().unwrap();
                *stack.last_mut().unwrap() += std::cmp::max(p * 2, 1);
            }
        }
    }
    stack[0]
}
