use itertools::join;
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
    graph.iter_mut().for_each(|x| x.sort_unstable());

    let mut path = vec![];
    dfs(&graph, None, 0, &mut path);

    println!("{}", join(path.iter().map(|&node| node + 1), " "));
}

fn dfs(graph: &Vec<Vec<usize>>, parent: Option<usize>, cur: usize, path: &mut Vec<usize>) {
    path.push(cur);

    for &next in &graph[cur] {
        if Some(next) != parent {
            dfs(graph, Some(cur), next, path);
            path.push(cur);
        }
    }
}
