use itertools::join;
use proconio::input;

fn main() {
    if let Some(ans) = solve() {
        println!("Yes\n{}", join(ans, " "));
    } else {
        println!("No");
    }
}

fn solve() -> Option<Vec<i64>> {
    input! {
        n: usize,
        ss: [i64; n],
    }

    let mut cc = vec![0; n + 2];
    for i in 2..(n + 2) {
        cc[i] = ss[i - 2] - cc[i - 2] - cc[i - 1];
    }

    let a = cc.iter().step_by(3).map(|&c| -c).max().unwrap();
    let b = cc.iter().skip(1).step_by(3).map(|&c| -c).max().unwrap();

    let mut aa = vec![0; n + 2];
    for i in 0..(n + 2) {
        aa[i] = if i % 3 == 0 {
            cc[i] + a
        } else if i % 3 == 1 {
            cc[i] + b
        } else {
            cc[i] - a - b
        }
    }

    if aa.iter().any(|&a| a < 0) {
        return None;
    }

    Some(aa)
}
