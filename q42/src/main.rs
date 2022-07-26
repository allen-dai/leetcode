fn main() {
    dbg!(trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]));
}

fn trap(height: Vec<i32>) -> i32 {
    let mut water = 0;
    let len = height.len();
    let mut l = 0;
    let mut r = len - 1;
    let mut max_l = height[0];
    let mut max_r = height[r];
    while l < r {
        if max_l <= max_r {
            l += 1;
            if height[l] > max_l {
                max_l = height[l];
            }
            water += max_l - height[l];
        } else if max_r < max_l {
            r -= 1;
            if height[r] > max_r {
                max_r = height[r];
            }
            water += max_r - height[r];
        }
    }
    return water;
}
