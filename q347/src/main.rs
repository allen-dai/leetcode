use std::collections::HashMap;
fn main() {
    dbg!(ans(vec![1, 1, 1, 2, 2, 3], 2));
}

fn ans(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut freq: HashMap<i32, i32> = HashMap::new();
    nums.into_iter().for_each(|n| *freq.entry(n).or_insert(0) += 1);
    let mut v: Vec<(i32, i32)> = freq.into_iter().collect();
    v.sort_by(|a,b| b.1.cmp(&a.1));
    return v.iter().take(k as usize).map(|p| p.0).collect();
}
