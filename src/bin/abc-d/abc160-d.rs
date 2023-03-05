use itertools::join;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, x, y): (usize, Usize1, Usize1),
    }

    let mut ans = vec![0_usize; n - 1];

    for i in 0..n {
        for j in (i + 1)..n {
            let dist = (j - i).min(abs_diff(i, x) + abs_diff(j, y) + 1);
            ans[dist - 1] += 1;
        }
    }

    println!("{}", join(ans, "\n"));
}

fn abs_diff(a: usize, b: usize) -> usize {
    if a >= b {
        a - b
    } else {
        b - a
    }
}
