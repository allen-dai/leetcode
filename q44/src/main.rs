fn wildcard(s: String, p: String) -> bool{
    let sl = s.len();
    let pl = p.len();
    let mut si: usize = 0;
    let mut m: usize = 0;
    let mut pi: usize = 0;
    let mut star_index: isize = -1;

    let sv: Vec<char> = s.chars().collect();
    let pv: Vec<char> = p.chars().collect();

    while si < sl {
        if pi < pl && (pv[pi] == '?' || sv[si] == pv[pi]) {
            si += 1;
            pi += 1;
        } 
        else if pi < pl && pv[pi] == '*' {
            star_index = pi as isize;
            m = si;
            pi += 1;
        }
        else if star_index != -1{
            pi = (star_index + 1) as usize;
            m += 1;
            si = m;
        }
        else {
            return false;
        }
    }
    while pi < pl && pv[pi] == '*'{
        pi += 1;
    }

    return pi == pl;
}

fn main() {
    let s = "aa".to_string();
    let p = "a".to_string();

    println!("{}", wildcard(s, p))
}
