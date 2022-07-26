use std::collections::HashMap;

fn main() {
    println!("{:?}", ans(vec![2, 7, 11, 15], 9));
}

fn ans(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, usize> = HashMap::new();

    for (i, n) in nums.iter().enumerate() {
        if map.contains_key(n) {
            let ans = *map.get(n).unwrap();
            if i <= ans{
                return vec![i as i32, ans as i32];
            }
            return vec![ans as i32, i as i32];
        } else {
            map.insert(target - n, i);
        }
    }

    return vec![];
}
