fn main() {}

fn ans(nums: Vec<i32>) -> i32 {
    let (mut l, mut r) = (0, nums.len() - 1);

    let mut m = (l + r) / 2;
    while l <= r {
        m = (l + r) / 2;
        if nums[l] < nums[r] || nums[m] < nums[r] {
            r = m;
        } else {
            l = m + 1;
        }
    }

    nums[m]
}

#[test]
fn t1() {
    assert_eq!(ans(vec![3, 4, 5, 1, 2]), 1);
    assert_eq!(ans(vec![5, 0, 2, 3, 4]), 0);
}

#[test]
fn t2() {
    assert_eq!(ans(vec![4, 5, 6, 7, 0, 1, 2]), 0);
}

#[test]
fn t3() {
    assert_eq!(ans(vec![11, 13, 15, 17]), 11);
}

#[test]
fn t4() {
    assert_eq!(ans(vec![2, 1]), 1);
    assert_eq!(ans(vec![1, 2]), 1);
}

#[test]
fn t5() {
    assert_eq!(ans(vec![1]), 1);
    assert_eq!(ans(vec![2]), 2);
}

#[test]
fn t6() {
    assert_eq!(ans(vec![10, 9, 8, 7, 1, 6]), 1);
}
