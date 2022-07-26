fn main() {}

fn ans(nums: Vec<i32>, target: i32) -> i32 {
    let (mut l, mut r) = (0, nums.len() - 1);

    while l <= r {
        let m = (l + r) / 2;
        if target == nums[m] {
            return m as i32;
        }

        if nums[l] <= nums[m] {
            if target > nums[m] || nums[l] > target {
                l = m + 1;
            } else {
                r = m - 1;
            }
        } else {
            if target < nums[m] || target > nums[r] {
                r = m - 1;
            } else {
                l = m + 1;
            }
        }
    }

    return -1;
}

#[test]
fn t1() {
    assert_eq!(ans(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
}

#[test]
fn t2() {
    assert_eq!(ans(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
}

#[test]
fn t3() {
    assert_eq!(ans(vec![1], 0), -1);
}

#[test]
fn t4() {
    assert_eq!(ans(vec![1, 3], 3), 1);
}

#[test]
fn t5() {
    assert_eq!(ans(vec![3, 1], 3), 0);
}

#[test]
fn t6() {
    assert_eq!(ans(vec![1, 3], 4), -1);
}

#[test]
fn t7() {
    assert_eq!(ans(vec![1, 3, 5], 0), -1);
}

#[test]
fn t8() {
    assert_eq!(ans(vec![1, 3, 5], 3), 1);
}

#[test]
fn t9() {
    assert_eq!(ans(vec![5, 1, 3], 3), 2);
}
