fn main() {}

struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        let len = nums.len();
        let mut temp: Vec<i32> = Vec::new();

        fn dfs(
            i: usize,
            temp: &mut Vec<i32>,
            len: usize,
            ans: &mut Vec<Vec<i32>>,
            nums: &Vec<i32>,
        ) {
            if i == len {
                ans.push((*temp).clone());
                return;
            }

            (*temp).push(nums[i]);
            dfs(i + 1, temp, len, ans, nums);

            temp.pop();
            dfs(i + 1, temp, len, ans, nums);
        }

        dfs(0, &mut temp, len, &mut ans, &nums);
        ans
    }
}

#[test]
fn test1() {
    let nums = vec![1, 2, 3];

    let ans = vec![
        vec![],
        vec![1],
        vec![2],
        vec![1, 2],
        vec![3],
        vec![1, 3],
        vec![2, 3],
        vec![1, 2, 3],
    ]
    .sort();
    let output = Solution::subsets(nums).sort();

    assert_eq!(output, ans);
}
