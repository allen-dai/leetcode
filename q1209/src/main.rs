fn main() {
    println!(
        "{:?}",
        ans("yfttttfbbbbnnnnffbgffffgbbbbgssssgthyyyy".to_string())
    );
}

fn tle_ans(s: String, k: i32) -> String {
    let mut l = s.len();

    if k > l as i32 {
        return s;
    }

    let mut s_v: Vec<char> = s.chars().collect();
    let mut does_contain: bool = true;

    while l as i32 - k > 0 && does_contain {
        does_contain = false;
        'outer: for i in 0..l + 1 - k as usize {
            //println!("{:?}", s_v);
            if s_v[i..i + k as usize].iter().all(|c| c.eq(&s_v[i])) {
                s_v.drain(i..i + k as usize);
                does_contain = true;
                l = s_v.len();
                break 'outer;
            }
        }
    }

    return s_v.iter().collect();
}
fn ans(s: String) -> String{
    let s_v:Vec<char> = s.chars().collect();
    let mut stack: Vec<char> = Vec::new();
    for c in s_v.iter(){
        if stack.len()==0{
            stack.push(*c);
        }
        else{
            if stack.last().unwrap() == c{
                stack.pop();
            }
            else{
                stack.push(*c);
            }
        }
    }
    return stack.iter().collect();

}

fn ans2(s: String, k: i32) -> String {
    let mut stack: Vec<(char, i32)> = Vec::new();
    let s_v: Vec<char> = s.chars().collect();

    for i in 0..s.len() {
        let c = &s_v[i];
        if stack.len() > 0 && stack.last().unwrap().0 == *c {
            if stack.last().unwrap().1 + 1 == k {
                stack.pop();
            } else {
                stack.last_mut().unwrap().1 += 1;
            }
        } else {
            stack.push((*c, 1));
        }
    }

    let mut ans: String = String::new();
    for i in 0..stack.len() {
        let c = stack[i].0.to_string();
        let length = stack[i].1;
        for j in 0..length {
            ans += &c;
        }
    }
    return ans;
}
