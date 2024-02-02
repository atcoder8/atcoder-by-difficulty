use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let sum = aa.iter().sum::<usize>();
    let x0 = sum - 2 * aa[1..].iter().step_by(2).sum::<usize>();
    let mut xx = vec![x0];
    for i in 0..n - 1 {
        xx.push(2 * aa[i] - xx[i]);
    }

    println!("{}", xx.iter().join(" "));
}
