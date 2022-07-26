use std::collections::VecDeque;

fn main() {
    println!("{:?}", ans(vec![1, 3, -1, -3, 5, 3, 6, 7], 3));
}

pub fn ans(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut queue = VecDeque::new();
    let mut ans = Vec::new();

    for i in 0..nums.len() {
        if !queue.is_empty() && *queue.front().unwrap() as i32 == i as i32 - k {
            queue.pop_front();
        };

        while !queue.is_empty() && nums[*queue.back().unwrap() as usize] < nums[i] {
            queue.pop_back();
        }

        queue.push_back(i);

        if i >= (k - 1) as usize {
            ans.push(nums[*queue.front().unwrap()])
        };
    }

    return ans;
}
