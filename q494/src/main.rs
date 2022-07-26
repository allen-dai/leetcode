fn main() {
    let nums: Vec<i32> = vec![0,0,0,0,0,0,0,0,1];
    println!("{}", ans(&nums, 1));
}

fn ans(nums: &Vec<i32>, s: i32) -> i32 {
    let l = nums.len();
    let mut temp = nums.clone();
    let mut ans = 0;
    for i in 0..l {
        for j in 0..l {
            if j != i {
                let t_sum: i32 = temp.iter().sum();
                println!("{:?}", temp);
                if t_sum == s {
                    ans += 1;
                }
                temp[j] = temp[j] * -1;
            }
        }

        for j in 0..l {
            if j != i {
                temp[j] = temp[j] * -1;
            }
        }
    }

    return ans;
}

fn subsetSum(nums: &Vec<i32>, s: i32) -> i32 {
    let mut dp: Vec<i32> = vec![0; (s + 1) as usize];
    dp[0] = 1;
    let mut i = s;

    for n in nums.iter() {
        while i >= *n {
            let j = i as usize;
            dp[j] += dp[j - *n as usize];
            i -= 1;
        }
    }
    return dp[s as usize];
}
