fn main() {
    dbg!(three_sum(vec![0, 0, 0, 0]));
}

fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let len = nums.len();
    let mut ans: Vec<Vec<i32>> = Vec::new();
    nums.sort();
    for i in 0..len {
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }
        let mut l = i + 1;
        let mut r = len - 1;

        while l < r {
            let sum = nums[i] + nums[l] + nums[r];
            if sum > 0 {
                r -= 1;
            } else if sum < 0 {
                l += 1;
            } else {
                ans.push(vec![nums[i], nums[l], nums[r]]);
                l += 1;
                while nums[l] == nums[l + 1] && l < r {
                    l += 1;
                }
            }
        }
    }

    return ans;
}
