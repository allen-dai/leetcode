fn main() {
    dbg!(ans(vec![1, 3, 5, 6], 5));
}

fn ans(nums: Vec<i32>, target: i32) -> i32 {
    match nums.binary_search(&4) {
        Ok(i) => return i as i32,
        Err(i) => return i as i32,
    }
}
