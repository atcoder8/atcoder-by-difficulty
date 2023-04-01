use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        (l, q): (usize, usize),
        cx: [(usize, usize); q],
    }

    let mut cut = BTreeSet::new();
    cut.insert(0);
    cut.insert(l);

    for &(c, x) in &cx {
        if c == 1 {
            cut.insert(x);
        } else {
            let left = cut.range(..x).last().unwrap();
            let right = cut.range(x..).next().unwrap();

            println!("{}", right - left);
        }
    }
}
