use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        abc: [(Usize1, Usize1, usize); m],
    }

    let mut distances = vec![vec![None; n]; n];
    for i in 0..n {
        distances[i][i] = Some(0);
    }
    for &(a, b, c) in &abc {
        distances[a][b] = Some(c);
    }

    let mut ans = 0;

    for mid in 0..n {
        for from in 0..n {
            for to in 0..n {
                if let (Some(dist1), Some(dist2)) = (distances[from][mid], distances[mid][to]) {
                    let candidate_dist = dist1 + dist2;
                    if distances[from][to].is_none()
                        || candidate_dist < distances[from][to].unwrap()
                    {
                        distances[from][to] = Some(candidate_dist);
                    }
                }

                ans += distances[from][to].unwrap_or(0);
            }
        }
    }

    println!("{}", ans);
}
