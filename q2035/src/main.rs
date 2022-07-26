fn all_sums(nums: Vec<i32>) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let half = nums.len() / 2;
    let (mut left, mut right): (Vec<Vec<i32>>, Vec<Vec<i32>>) =
        (vec![vec![]; half + 1], vec![vec![]; half + 1]);

    for mask in 0..1 << half {
        let (mut sz, mut l, mut r): (usize, i32, i32) = (0, 0, 0);
        for i in 0..half {
            if mask & (1 << i) != 0 {
                sz += 1;
                l += nums[i];
                r += nums[i + half];
            }
        }
        left[sz].push(l);
        right[sz].push(r);
    }

    for sz in 0..half {
        right[sz].sort();
    }

    return (left, right);
}

fn main() {
    let nums: Vec<i32> = vec![2, -1, 0, 4, -2, -9];

    let sum: i32 = nums.iter().sum();

    let half = nums.len() / 2;

    let (left, right) = all_sums(nums);

    let res = std::cmp::min(
        (sum - 2 * left[half][0]).abs(),
        (sum - 2 * right[half][0]).abs(),
    );


    for sz in 1..half{
        for n in left[sz].iter(){
        }
    }

    println!("\n{}\n{:?} \n{:?}", sum, left, right);
}
