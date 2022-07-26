fn main() {
    let times = vec![vec![2, 1, 1], vec![2, 3, 1], vec![3, 4, 1]];
    let n = 4;
    let k = 2;
    println!("{:?}", network_delay_time(times, n, k));
}

pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
    let n = n as usize;
    let k = k as usize;
    let adj_vec = {
        let mut res = vec![vec![]; n + 1];
        for time in times {
            let u = time[0] as usize;
            let v = time[1] as usize;
            let w = time[2] as usize;
            res[u].push((v, w));
        }
        res
    };
    println!("{:?}", adj_vec);
    return 0;
}
