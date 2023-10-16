use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n - 1],
    }

    let mut graph = vec![vec![]; n];
    for &(a, b) in &ab {
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut dists = vec![n; n];
    let mut stack = vec![(0, 0)];
    while let Some((cur, dist)) = stack.pop() {
        if dist >= dists[cur] {
            continue;
        }

        dists[cur] = dist;

        for &next in &graph[cur] {
            stack.push((next, dist + 1));
        }
    }

    let mut path = vec![n - 1];
    let mut cur = n - 1;
    while cur != 0 {
        let prev = *graph[cur]
            .iter()
            .find(|&&prev| dists[prev] == dists[cur] - 1)
            .unwrap();
        path.push(prev);
        cur = prev;
    }
    path.reverse();

    let stop = path[(path.len() + 1) / 2];

    let mut visited = vec![false; n];
    let mut stack = vec![0];
    while let Some(cur) = stack.pop() {
        if cur == stop || visited[cur] {
            continue;
        }

        visited[cur] = true;

        stack.extend(graph[cur].clone());
    }

    let visited_num = visited.iter().filter(|&&x| x).count();

    println!(
        "{}",
        if visited_num > n / 2 {
            "Fennec"
        } else {
            "Snuke"
        }
    );
}
