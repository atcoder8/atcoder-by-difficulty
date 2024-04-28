use itertools::{chain, Itertools};
use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        s: String,
    }

    let unhappiness_num = chain!(['R'], s.chars(), ['L'])
        .tuple_windows()
        .filter(|&(dir1, dir2, dir3)| dir2 == 'L' && dir1 == 'R' || dir2 == 'R' && dir3 == 'L')
        .count();
    let opposite_num = s
        .chars()
        .tuple_windows()
        .positions(|(dir1, dir2)| dir1 != dir2)
        .count();

    let ans = n - (unhappiness_num - opposite_num.min(2 * k));
    println!("{}", ans);
}
