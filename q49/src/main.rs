fn main() {
    println!(
        "{:?}",
        ans2(vec![
            "eat".to_string(),
            "".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "".to_string(),
            "a".to_string(),
            "bat".to_string(),
            "".to_string(),
        ])
    );

    println!("{:?}", is_ana("ac".to_string(), "acc".to_string()));
}

fn ans(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut ans: Vec<Vec<String>> = Vec::new();
    let mut added: bool;
    for s in strs.iter() {
        added = false;
        'outer: for (i, v) in ans.iter().enumerate() {
            if is_ana(s.to_string(), v[0].to_string()) && s!="" && v[0] != ""{
                ans[i].push(s.to_string());
                added = true;
                break 'outer;
            }
            else if s=="" && v[0] == ""{
                ans[i].push(s.to_string());
                added = true;
                break 'outer;
            }
        }
        if !added {
            ans.push(vec![s.to_string()]);
        }
    }

    return ans;
}

fn is_ana(s: String, t: String) -> bool {
        let mut buffer = [0isize; 0x80];
        let (s, t) = (s.as_bytes(), t.as_bytes());
        for (&a, &b) in s.iter().zip(t.iter()){
            buffer[usize::from(a)] += 1;
            buffer[usize::from(b)] -= 1;
        }
        buffer.iter().all(|&x|x==0)
}


fn ans2(strs: Vec<String>) -> Vec<Vec<String>> {
    use std::collections::HashMap;
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    for s in strs{
        let signature: Vec<u32> = {
            let mut temp: Vec<u32> = vec![0; 26];
            for c in s.chars(){
                let ascii = c as usize - 'a' as usize;
                temp[ascii] += 1;
            }
            temp
        };

        let hash = format!("{:?}", signature);
        map.entry(hash).or_insert(vec![]).push(s);
    }

    return map.into_values().collect();
}
