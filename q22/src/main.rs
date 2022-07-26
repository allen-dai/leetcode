fn dfs(left: i32, right: i32, ans: &mut Vec<String>, s: String) {
    println!("{:?}", s);
    if right < left {
        return;
    }
    if right + left == 0 {
        ans.push(s);
        return;
    }

    if left > 0 {
        dfs(left - 1, right, ans, s.clone() + "(");
    }
    if right > 0 {
        dfs(left, right - 1, ans, s.clone() + ")");
    }
}

fn test(n: i32) -> Vec<String> {
    if n == 0 {
        return vec![];
    }
    let (mut left, mut right, mut ans): (i32, i32, Vec<String>) = (n, n, Vec::new());
    dfs(left, right, &mut ans, String::from(""));
    return ans;
}

fn main() {
    println!("{:?}", test(3));
}
