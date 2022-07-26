fn main() {
    //println!("{:?}", ans(vec![2, 4]));
    println!("{:?}", ans(vec![2, 1, 5, 6, 2, 3]));
}

fn ans(heights: Vec<i32>) -> i32 {
    let mut res = 0;
    let mut stack: Vec<(usize, i32)> = Vec::new();
    for (I, H) in heights.iter().enumerate() {
        let mut start = I;
        while !stack.is_empty() && stack.last().unwrap().1 > *H {
            let (i, h) = stack.pop().unwrap();
            res = i32::max(res, (I as i32 - i as i32) * h);
            start = i;
        }
        stack.push((start, *H));
    }
    for (_, (I, H)) in stack.iter().enumerate() {
        res = i32::max(res, H * (heights.len() - I) as i32)
    }
    res
}
