fn main() {
    println!("{}", ans(")()())".to_string()));
}

fn ans(s: String) -> i32 {
    let mut stack = Vec::new();
    let mut res = 0;
    let mut temp = 0;
    let mut cont = false;
    for c in s.chars() {
        if c == '(' {
            stack.push(c);
            cont = false;
            temp = 0;
        } else {
            cont = true;
            if !stack.is_empty() {
                stack.pop();
                if cont {
                    temp += 2;
                    if temp > res{
                        res = temp;
                    }
                }
            }
        }
    }
    res
}
