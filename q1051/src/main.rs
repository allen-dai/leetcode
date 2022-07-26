fn main() {
    println!("{:?}", ans(vec![1, 1, 4, 2, 1, 3]));
}

fn ans(heights: Vec<i32>) -> i32 {
    let mut temp = heights.clone();
    temp.sort();
    let mut res = 0;
    for i in 0..temp.len(){
        if temp[i] != heights[i]{
            res += 1;
        }
    }
    res
}
