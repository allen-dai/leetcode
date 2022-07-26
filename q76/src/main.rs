use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

fn main() {
    dbg!(min_window(
        "aaaaaaaaaaaabbbbbcdd".to_string(),
        "abcdd".to_string()
    ));
}

fn min_window(s: String, t: String) -> String {
    if t.len() == 0 || t.len() > s.len() {
        return "".to_string();
    }
    let mut t_map = HashMap::new();
    for c in t.chars() {
        *t_map.entry(c).or_insert(1) += 1;
    }

    let mut window = HashMap::new();
    let mut have = 0;
    let need = t_map.len();
    let mut l = 0;
    let (mut res, mut resLen) = (vec![], usize::MAX);
    let sb = s.as_bytes();

    for (i, c) in s.chars().enumerate() {
        *window.entry(c).or_insert(1) += 1;
        if t_map.contains_key(&c) && window.get(&c) == t_map.get(&c) {
            have += 1;
        }
        while have == need {
            if (i - l + 1) < resLen {
                res = vec![l, i];
                resLen = i - l + 1;
            }
            let r_c: &char = &(sb[l] as char);
            *window.get_mut(r_c).unwrap() -= 1;
            if t_map.contains_key(r_c) && window[r_c] < t_map[r_c] {
                have -= 1;
            }
            l += 1;
        }
    }

    if resLen == usize::MAX {
        return "".to_string();
    }

    res.sort();

    return s[res[0]..res[1]+1].to_string();
}
