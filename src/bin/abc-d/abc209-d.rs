use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, q): (usize, usize),
        ab: [(Usize1, Usize1); n - 1],
        cd: [(Usize1, Usize1); q],
    }

    let mut graph = vec![vec![]; n];
    for &(a, b) in &ab {
        graph[a].push(b);
        graph[b].push(a);
    }
    graph.iter_mut().for_each(|x| x.sort_unstable());

    let mut distances = vec![None; n];
    distances[0] = Some(0);
    let mut stack = vec![0];

    while let Some(node) = stack.pop() {
        for &next_node in &graph[node] {
            if distances[next_node].is_none() {
                distances[next_node] = Some(distances[node].unwrap() + 1);
                stack.push(next_node);
            }
        }
    }

    for &(c, d) in &cd {
        println!(
            "{}",
            if distances[c].unwrap() % 2 == distances[d].unwrap() % 2 {
                "Town"
            } else {
                "Road"
            }
        );
    }
}
