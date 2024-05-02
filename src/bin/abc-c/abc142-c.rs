use itertools::{enumerate, Itertools};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        aa: [Usize1; n],
    }

    let mut positions = vec![0; n];
    for (i, &a) in enumerate(&aa) {
        positions[a] = i;
    }

    println!("{}", positions.iter().map(|pos| pos + 1).join(" "));
}
