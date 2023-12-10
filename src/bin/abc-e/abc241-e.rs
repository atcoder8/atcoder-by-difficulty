use itertools::enumerate;
use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        aa: [usize; n],
    }

    let log = k.ilog2() as usize;

    let mut doubling = vec![vec![0; n]; log + 1];
    for (i, &a) in enumerate(&aa) {
        doubling[0][i] = a;
    }
    for exp in 0..log {
        for i in 0..n {
            let cnt1 = doubling[exp][i];
            let cnt2 = doubling[exp][(i + cnt1) % n];
            doubling[exp + 1][i] = cnt1 + cnt2;
        }
    }

    let mut ans = 0;
    for exp in 0..=log {
        if k >> exp & 1 == 1 {
            ans += doubling[exp][ans % n];
        }
    }

    println!("{}", ans);
}
