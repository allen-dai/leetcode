fn main() {
    println!("{}", find_duplicate(vec![3, 1, 3, 4, 2]));
}

fn find_duplicate(nums: Vec<i32>) -> i32 {
    use std::collections::HashSet;
    let mut seen = HashSet::new();
    for n in nums.iter(){
        if seen.contains(n){
            return *n;
        }

        seen.insert(n);
    }
    0
}
