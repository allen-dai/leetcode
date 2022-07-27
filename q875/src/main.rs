fn main() {}

fn ans(piles: Vec<i32>, h: i32) -> i32 {
    let max = piles.iter().max().unwrap();
    let mut res = *max;
    if h == piles.len() as i32 {
        return *max;
    }

    let hrs = |f: i32| {
        piles
            .iter()
            .map(|p| {
                if p % f > 0 {
                    return p / f + 1;
                }
                p / f
            })
            .sum::<i32>()
    };

    let (mut l, mut r) = (1, *max);
    let mut m = (l + r) / 2;
    while l < r {
        m = (l + r) / 2;
        let hr = hrs(m);
        if hr <= h {
            res = i32::min(res, m);
            r = m;
        } else {
            l = m + 1;
        }
    }

    res
}

#[test]
fn t1() {
    let t1: Vec<i32> = vec![3, 6, 7, 11];
    let t1h = 8;
    let t1a = 4;
    assert_eq!(ans(t1, t1h), t1a);
}

#[test]
fn t2() {
    let t2: Vec<i32> = vec![30, 11, 23, 4, 20];
    let t2h = 5;
    let t2a = 30;
    assert_eq!(ans(t2, t2h), t2a);
}

#[test]
fn t3() {
    let t3: Vec<i32> = vec![30, 11, 23, 4, 20];
    let t3h = 6;
    let t3a = 23;
    assert_eq!(ans(t3, t3h), t3a);
}

#[test]
fn t4() {
    let t4: Vec<i32> = vec![805306368, 805306368, 805306368];
    let t4h = 1000000000;
    let t4a = 3;
    assert_eq!(ans(t4, t4h), t4a);
}

#[test]
fn t5() {
    let t4: Vec<i32> = vec![1000000000, 1000000000];
    let t4h = 3;
    let t4a = 1000000000;
    assert_eq!(ans(t4, t4h), t4a);
}
