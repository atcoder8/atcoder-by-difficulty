use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, t): (usize, usize),
        tt: [usize; n],
    }

    let ans = tt
        .iter()
        .tuple_windows()
        .map(|(t1, t2)| t.min(t2 - t1))
        .sum::<usize>()
        + t;
    println!("{}", ans);
}
