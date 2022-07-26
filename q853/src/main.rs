use std::collections::HashMap;

fn main() {
    println!("{:?}", ans(12, vec![10, 8, 0, 5, 3], vec![2, 4, 1, 1, 3]));
}

fn ans(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
    let mut car = Vec::new();

    for i in 0..position.len() {
        car.push((position[i], speed[i]));
    }

    car.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    car.reverse();

    let mut stack = Vec::new();

    for (p, s) in car {
        let t = (target - p) as f32 / s as f32;
        while !stack.is_empty() && *stack.last().unwrap() >= t{
            stack.pop();
        }
        stack.push(t);
    }

    return stack.len() as i32;
}
