fn main() {
    println!("{:?}", ans(vec![73, 74, 75, 71, 69, 72, 76, 73]));
}

fn ans(temperatures: Vec<i32>) -> Vec<i32> {
    let mut stack: Vec<(i32, usize)> = Vec::new();
    let mut res = vec![0 as i32; temperatures.len()];

    for (i, &temp) in temperatures.iter().enumerate() {
        while !stack.is_empty() && temp > stack.last().unwrap().0 {
            let (st, si) = stack.pop().unwrap();
            res[si] = (i - si) as i32;
        }

        stack.push((temp, i));
    }

    return res;
}
