fn main() {
    dbg!(ans("23".to_string()));
}

fn ans(digits: String) -> Vec<String> {

    if digits.len() == 0{
        return vec![];
    }
    let keys: Vec<&str> = vec![
        "", "", "abc", "def", "ghi", "jkl", "mno", "pgrs", "tuv", "wxyz",
    ];
    let combs: Vec<&str> = digits.chars().map(|d| keys[(d as usize - 48)]).collect();
    let range: usize = combs.iter().map(|c| c.len()).product();
    let mut ans: Vec<String> = vec!["".to_string(); range];
    let mut rep = range;
    for comb in combs {
        rep /= comb.len();
        let mut index = 0;
        let mut couter = 0;
        for i in 0..range {
            if couter == rep {
                index += 1;
                couter = 0;
                if index == comb.len() {
                    index = 0;
                }
            }
            let curr = comb.as_bytes()[index] as char;
            ans[i] += &curr.to_string();
            couter += 1;
        }
    }
    return ans;
}
