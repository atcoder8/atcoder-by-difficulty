use itertools::Itertools;
use proconio::{input, marker::Bytes};

fn main() {
    input! {
        n: Bytes,
    }

    let digits = n.iter().map(|&c| (c - b'0') as usize).collect_vec();

    let mut greater = 1;
    let mut equal = 0;

    for &d in &digits {
        let next_greater = (greater + 9 - d).min(greater + d + 2).min(equal + d + 1);
        let next_equal = (equal + d).min(greater + d + 1);

        greater = next_greater;
        equal = next_equal;
    }

    let ans = (greater + 1).min(equal);
    println!("{}", ans);
}
