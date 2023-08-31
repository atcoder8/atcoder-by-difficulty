use itertools::join;
use proconio::input;

fn main() {
    input! {
        n: usize,
        ss: [usize; n],
        mut tt: [usize; n],
    }

    for i in 0..(2 * n - 2) {
        let (from, to) = (i % n, (i + 1) % n);
        tt[to] = tt[to].min(tt[from] + ss[from]);
    }

    println!("{}", join(tt, "\n"));
}
