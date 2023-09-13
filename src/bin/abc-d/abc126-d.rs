use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        uvw: [(Usize1, Usize1, usize); n - 1],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v, w) in &uvw {
        graph[u].push((v, w));
        graph[v].push((u, w));
    }

    let mut dists = vec![None; n];
    let mut stack: Vec<(usize, usize)> = vec![(0, 0)];
    while let Some((cur, dist)) = stack.pop() {
        let cur_dist = &mut dists[cur];
        if cur_dist.is_some() {
            continue;
        }

        *cur_dist = Some(dist);

        for &(next, edge_dist) in &graph[cur] {
            stack.push((next, dists[cur].unwrap() + edge_dist));
        }
    }

    for dist in dists {
        println!("{}", dist.unwrap() % 2);
    }
}
