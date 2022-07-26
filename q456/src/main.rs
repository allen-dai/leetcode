fn main() {
    println!("{}", ans2(vec![3,1,4,2]));
}

fn ans(nums: Vec<i32>) -> bool {
    if nums.len() < 3 {
        return false;
    }

    let mut nms = nums.clone();
    nms.dedup();

    for i in 0..nms.len() - 2 {
        for j in i + 1..nms.len() - 1 {
            for k in j + 1..nms.len() {
                if nms[i] < nms[k] && nms[i] < nms[j] && nms[k] < nms[j] {
                    return true;
                }
            }
        }
    }

    return false;
}

fn ans2(nums: Vec<i32>) -> bool {
    let l = nums.len();
    if l < 3 {
        return false;
    }
    let mut stack: Vec<i32> = Vec::new();
    let mut k_v = i32::MIN;
    for i in (0..l).rev() {
        let curr = nums[i];
        if curr < k_v{
            return true
        }
        while !stack.is_empty() && curr > *stack.last().unwrap(){
            k_v = stack.pop().unwrap();
        }
        stack.push(curr);
    }

    return false;
}
