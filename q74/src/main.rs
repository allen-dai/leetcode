fn main() {
}

fn ans(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let (mut l, mut L) = (0, 0);
    let (mut r, mut R) = (matrix[0].len(), matrix.len());
    let rl = matrix[0].len() - 1;
    let mut M_R = (R + L) / 2;
    while L < R {
        M_R = (R + L) / 2;
        if matrix[M_R][0] <= target && matrix[M_R][rl] >= target {
            break;
        }
        if matrix[M_R][0] > target {
            R = M_R;
        } else if matrix[M_R][0] < target {
            L = M_R + 1;
        } else {
            break;
        }
    }

    while l < r {
        let m = (r + l) / 2;
        if matrix[M_R][m] > target {
            r = m;
        } else if matrix[M_R][m] < target {
            l = m + 1;
        } else {
            return true;
        }
    }

    false
}

#[test]
fn some_test() {
    let g = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 50]];
    let s:Vec<i32> = vec![1, 3, 5, 7, 10, 11, 16, 20, 23, 30, 34, 50];
    for i in s.iter(){
        assert_eq!(ans(g.clone(), *i), true);
    }
}
