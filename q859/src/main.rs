fn main() {
    println!("{:?}", ans("aa".to_string(), "aa".to_string()));
}

fn ans(s: String, goal: String) -> bool {
    if goal.len() != s.len() {
        return false;
    }

    let mut diff = 0;
    let sb = s.as_bytes();
    let gb = goal.as_bytes();
    let mut rep = 0;
    for i in 0..sb.len() {
        if i + 1 < sb.len() {
            if sb[i] == sb[i + 1] {
                rep += 1;
            }
        }
        if sb[i] != gb[i] {
            diff += 1;
        }
    }
    diff == 2 || rep > 0
}
