use std::collections::HashMap;

fn main() {
    println!("{}", ans("cat".to_string(), "car".to_string()));
}

fn ans(s: String, t: String) -> bool{
    let mut map = HashMap::new();
    s.chars().for_each(|c| *map.entry(c).or_insert(0) += 1);
    t.chars().for_each(|c| *map.entry(c).or_insert(0) += 1);
    println!("{:?}", map);
    return map.into_values().all(|v| v == 0);
}
