fn main() {
    dbg!(ans("AABABBA".to_string(), 1));
}

use std::cmp::max;
use std::collections::HashMap;
fn ans(s: String, k: i32) -> i32 {
    let (mut l, mut r, mut ans, mut max_f) = (0, 0, 0, 0);
    let mut map = HashMap::new();
    for c in s.chars() {
        let curr_f = map.entry(c).or_insert(0);
        *curr_f += 1;
        max_f = max(*curr_f, max_f);
        while r - l + 1 - max_f > k {
            *map.get_mut(&(s.as_bytes()[l as usize] as char)).unwrap() -= 1;
            l += 1;
        }
        ans = max(ans, r - l + 1);
        r += 1;
    }
    return ans;
}
