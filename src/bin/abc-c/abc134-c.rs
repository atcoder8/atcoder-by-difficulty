use itertools::{chain, Itertools};
use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let max_pos = aa.iter().position_max().unwrap();
    let max = aa[max_pos];
    let second_max = *chain(&aa[..max_pos], &aa[max_pos + 1..]).max().unwrap();

    let ans = aa
        .iter()
        .map(|&a| if a == max { second_max } else { max })
        .join("\n");
    println!("{}", ans);
}
