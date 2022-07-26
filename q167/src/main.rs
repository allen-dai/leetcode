fn main() {
    dbg!(two_sum(vec![2, 3, 4], 6));
}

fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut l = 0;
    let mut r = numbers.len() - 1;

    for _ in 0..numbers.len(){
        let sum = numbers[l] + numbers[r];
        if sum == target{
            return vec![(l+1) as i32, (r+1) as i32];
        }
        if sum > target{
            r -= 1;
        }
        else{
            l += 1;
        }
    }

    return vec![];
}
