use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

fn ans(nums: Vec<i32>) -> bool {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for n in nums.iter() {
        if map.contains_key(n){
            return true;
        }
        map.insert(*n, 0);
    }
    return false;
}
