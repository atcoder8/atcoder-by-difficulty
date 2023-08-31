use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        abc: [(Usize1, Usize1, usize); m],
    }

    let mut dists: Vec<Vec<Option<usize>>> = vec![vec![None; n]; n];
    for &(a, b, c) in &abc {
        dists[a][b] = Some(c);
        dists[b][a] = Some(c);
    }
    for mid in 0..n {
        for from in 0..n {
            for to in 0..n {
                let (dist1, dist2) = match (dists[from][mid], dists[mid][to]) {
                    (Some(dist1), Some(dist2)) => (dist1, dist2),
                    _ => continue,
                };

                let candidate_dist = dist1 + dist2;
                let dist = &mut dists[from][to];
                if dist.is_none() || candidate_dist < dist.unwrap() {
                    *dist = Some(candidate_dist);
                }
            }
        }
    }

    let mut ans = 0;
    for &(a, b, c) in &abc {
        ans += (0..n).any(|mid| dists[a][mid].unwrap() + dists[mid][b].unwrap() <= c) as usize;
    }
    println!("{}", ans);
}
