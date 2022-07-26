fn main() {
    let input = vec![
        "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+",
    ];
    let mut tokens: Vec<String> = Vec::new();
    for c in input {
        tokens.push(c.to_string());
    }

    dbg!(ans(tokens));
}

pub fn ans(tokens: Vec<String>) -> i32 {
    let mut stack = Vec::new();

    for s in tokens {
        match s.as_str() {
            "+" => {
                let r = stack.pop().unwrap();
                let l = stack.pop().unwrap();
                stack.push(l + r);
            }
            "-" => {
                let r = stack.pop().unwrap();
                let l = stack.pop().unwrap();
                stack.push(l - r);
            }
            "*" => {
                let r = stack.pop().unwrap();
                let l = stack.pop().unwrap();
                stack.push(l * r);
            }
            "/" => {
                let r = stack.pop().unwrap();
                let l = stack.pop().unwrap();
                stack.push(l / r);
            }

            _ => {
                stack.push(s.parse::<i32>().unwrap());
            }
        }
        println!("{:?}", stack);
    }
    return *stack.last().unwrap();
}
