use itertools::chain;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        println!("{}", if solve() { "Yes" } else { "No" });
    }
}

fn solve() -> bool {
    input! {
        n: usize,
        mut aa: [Usize1; n],
        mut bb: [Usize1; n],
    }

    if aa == bb {
        return true;
    }

    let compress = |seq: &mut Vec<usize>| {
        seq.dedup();

        if seq.len() >= 2 && seq.first() == seq.last() {
            seq.pop();
        }
    };

    compress(&mut aa);
    compress(&mut bb);

    if bb.len() == n {
        return false;
    }

    let is_ok = |shift: usize| {
        let mut iter_aa = aa.iter();
        for &b in chain!(&bb[shift..], &bb[..shift]) {
            if iter_aa.find(|&&a| a == b).is_none() {
                return false;
            }
        }

        true
    };

    (0..bb.len()).any(|start| is_ok(start))
}
