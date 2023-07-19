use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        aaa: [[usize; n]; n],
    }

    let is_ok = |x: usize| {
        let mut acc = vec![vec![0; n + 1]; n + 1];
        for i in 0..n {
            for j in 0..n {
                acc[i + 1][j + 1] = (aaa[i][j] >= x) as usize;
            }
        }
        for i in 0..=n {
            for j in 0..n {
                acc[i][j + 1] += acc[i][j];
            }
        }
        for i in 0..n {
            for j in 0..=n {
                acc[i + 1][j] += acc[i][j];
            }
        }

        (0..=(n - k)).all(|top| {
            (0..=(n - k)).all(|left| {
                acc[top + k][left + k] - acc[top][left + k] - acc[top + k][left] + acc[top][left]
                    >= k.pow(2) / 2 + 1
            })
        })
    };

    let mut ok = 0;
    let mut ng: usize = 10_usize.pow(9) + 1;

    while ng - ok > 1 {
        let mid = (ok + ng) / 2;

        if is_ok(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}
