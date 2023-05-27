use std::collections::BTreeSet;

use proconio::input;

const N: usize = 1_usize << 20;

fn main() {
    input! {
        q: usize,
        tx: [(usize, usize); q],
    }

    let mut aa: Vec<Option<usize>> = vec![None; N];
    let mut indexes: BTreeSet<usize> = (0..N).collect();

    for &(t, x) in &tx {
        let r = x % N;

        if t == 1 {
            let h = if let Some(&h) = indexes.range(r..).next() {
                h
            } else {
                *indexes.iter().next().unwrap()
            };
            aa[h] = Some(x);
            indexes.remove(&h);
        } else {
            if let Some(a) = aa[r] {
                println!("{}", a);
            } else {
                println!("-1");
            }
        }
    }
}
