use fixedbitset::FixedBitSet;
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut ab: [(usize, usize); n],
        mut cd: [(usize, usize); n],
    }

    ab.sort_unstable();
    cd.sort_unstable();

    let mut pair_num = 0;
    let mut red_idx = 0;
    let mut red_candidates = FixedBitSet::with_capacity(2 * n);
    for &(c, d) in &cd {
        while red_idx < n && ab[red_idx].0 < c {
            red_candidates.insert(ab[red_idx].1);
            red_idx += 1;
        }

        if let Some(pos) = (0..d).rev().find(|&b| red_candidates[b]) {
            pair_num += 1;
            red_candidates.set(pos, false);
        }
    }

    println!("{}", pair_num);
}
