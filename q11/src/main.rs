fn main() {
    dbg!(max_area(vec![1, 1]));
}

fn max_area(height: Vec<i32>) -> i32 {
    let mut a: i32 = 0;
    let mut w = height.len();

    let (mut l, mut r) = (0, w-1);

    while l < r {
        w-=1;
        if height[l] < height[r] {
            a = a.max(height[l] * w as i32);
            l += 1;
        } else {
            a = a.max(height[r] * w as i32);
            r -= 1;
        }
    }

    return a;
}
