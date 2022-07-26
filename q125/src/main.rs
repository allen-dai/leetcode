fn main() {
    dbg!(ans("A man, a plan, a canal: Panama".to_string()));
}

fn ans(s: String) -> bool {
    let l = s.len();
    if l < 1 {
        return true;
    }
    let mut s_v: Vec<String> = Vec::new();
    for c in s.chars() {
        if c.is_alphanumeric() {
            s_v.push(c.to_lowercase().to_string());
        }
    }
    let l = s_v.len();
    let half = l / 2;
    let mut left = String::new();
    let mut right = String::new();
    let mut r = l - 1;
    for i in 0..half {
        left += &s_v[i];
        right += &s_v[r];
        r -= 1;
    }
    if left.eq(&right) {
        return true;
    }
    return false;
}
