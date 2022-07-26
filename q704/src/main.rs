fn main() {
    println!("{:?}", ans(vec![-1, 0, 3, 5, 9, 12], 12));
}

fn ans(nums: Vec<i32>, target: i32) -> i32 {
    let mut l = 0;
    let mut r = nums.len();

    while l < r {
        let m = (l + r) / 2;
        if nums[m] > target {
            r = m;
        } else if nums[m] < target {
            l = m + 1;
        } else {
            return m as i32;
        }
    }
    -1
}
