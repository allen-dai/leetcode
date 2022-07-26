fn main() {
    println!("{}", ans("saeed".to_string(), "ssaaedd".to_string()));
}

fn ans2(name: String, typed: String) -> bool {
    if name.len() > typed.len() {
        return false;
    }
    let mut name_v: Vec<char> = name.chars().collect();
    let mut typed_v: Vec<char> = name.chars().collect();
    name_v.sort();
    typed_v.sort();
    let len = name_v.len();
    name_v[..] == typed_v[0..len]
}

fn ans(name: String, typed: String) -> bool {
    if name.len() > typed.len() {
        return false;
    }
    let mut index = 0;
    for c in name.chars() {
        if typed.as_bytes()[index] as char != c {
            return false;
        }
        while typed.as_bytes()[index] as char == c && index < typed.len() - 1 {
            index += 1;
        }
    }
    true
}
