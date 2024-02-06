use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let mut balls = vec![false; n];
    for i in (1..=n).rev() {
        let cnt = (2 * i..=n).step_by(i).filter(|&j| balls[j - 1]).count();
        balls[i - 1] = cnt % 2 != aa[i - 1];
    }

    let ans = (1..=n).filter(|&i| balls[i - 1]).collect_vec();
    println!("{}\n{}", ans.len(), ans.iter().join(" "));
}
