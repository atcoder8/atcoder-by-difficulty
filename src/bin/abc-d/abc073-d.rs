use itertools::{iproduct, Itertools};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m, r): (usize, usize, usize),
        rr: [Usize1; r],
        abc: [(Usize1, Usize1, usize); m],
    }

    let mut dists = vec![vec![None; n]; n];
    for i in 0..n {
        dists[i][i] = Some(0);
    }
    for &(a, b, c) in &abc {
        dists[a][b] = Some(c);
        dists[b][a] = Some(c);
    }
    for (mid, from, to) in iproduct!(0..n, 0..n, 0..n) {
        if let (Some(dist1), Some(dist2)) = (dists[from][mid], dists[mid][to]) {
            update_cost(&mut dists[from][to], dist1 + dist2);
        }
    }

    let calc_sum_dist = |perm: &[usize]| {
        perm.iter()
            .tuple_windows()
            .map(|(&city1, &city2)| dists[city1][city2].unwrap())
            .sum::<usize>()
    };

    let ans = rr
        .into_iter()
        .permutations(r)
        .map(|perm| calc_sum_dist(&perm))
        .min()
        .unwrap();
    println!("{}", ans);
}

/// Updates the minimum cost.
/// If `cost` is `None`, always updated to the candidate cost.
///
/// # Arguments
///
/// * `cost` - Reference variable for the cost to be updated.
/// * `cand_cost` - Candidate cost to update.
pub fn update_cost<T>(cost: &mut Option<T>, cand_cost: T) -> bool
where
    T: PartialOrd,
{
    if cost.as_ref().is_some_and(|cost| cost <= &cand_cost) {
        return false;
    }

    *cost = Some(cand_cost);

    true
}
