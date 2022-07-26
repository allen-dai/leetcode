fn main() {
    dbg!(ans("abcabcbb".to_string()));
}

fn ans(s: String) -> i32 {
    use std::cmp::max;
    let mut map = std::collections::HashMap::new();
    let mut l = -1;
    let mut r = 0;
    let mut ans = 0;
    for c in s.chars() {
        if let Some(curr_repeat_i) = map.insert(c, r) {
            l = max(curr_repeat_i, l);
        }
        ans = max(ans, r - l);
        r += 1;
    }
    return ans;
}
