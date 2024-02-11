use std::collections::BTreeSet;

use itertools::enumerate;
use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let mut set = BTreeSet::<(usize, usize)>::new();
    for (i, &a) in enumerate(&aa) {
        if let Some(&prev) = set.range(..(a, 0)).next_back() {
            set.remove(&prev);
        }

        set.insert((a, i));
    }

    println!("{}", set.len());
}
