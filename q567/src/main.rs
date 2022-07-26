use std::collections::HashSet;

fn main() {
    dbg!(check_inclusion("ab".to_string(), "eidbaooo".to_string()));
}

fn check_inclusion(s1: String, s2: String) -> bool {
    if s1.len() > s2.len() {
        return false;
    }

    let mut a1 = [0 as usize; 26];
    let mut a2 = [0 as usize; 26];

    for c in s1.chars() {
        a1[(c as u8 - b'a') as usize] += 1;
    }

    for (i, c) in s2.chars().enumerate() {
        a2[(c as u8 - b'a') as usize] += 1;
        if i >= s1.len() {
            a2[(s2.as_bytes()[i - s1.len()] - b'a') as usize] -= 1;
        }

        if a1 == a2 {
            return true;
        }
    }

    return false;
}
