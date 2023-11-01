use itertools::iproduct;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        abt: [(Usize1, Usize1, usize); m],
    }

    let mut dists = vec![vec![None; n]; n];
    for i in 0..n {
        dists[i][i] = Some(0);
    }

    for &(a, b, t) in &abt {
        dists[a][b] = Some(t);
        dists[b][a] = Some(t);
    }

    for (mid, from, to) in iproduct!(0..n, 0..n, 0..n) {
        if let (Some(dist1), Some(dist2)) = (dists[from][mid], dists[mid][to]) {
            if dists[from][to].is_none() || dist1 + dist2 < dists[from][to].unwrap() {
                dists[from][to] = Some(dist1 + dist2);
            }
        }
    }

    let ans = (0..n)
        .map(|from| (0..n).map(|to| dists[from][to].unwrap()).max().unwrap())
        .min()
        .unwrap();
    println!("{}", ans);
}
