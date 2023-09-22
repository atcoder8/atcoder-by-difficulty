use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        uv: [(Usize1, Usize1); n - 1],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v) in &uv {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut sections = vec![(0, 0); n];
    dfs(&graph, &mut sections, None, 0, &mut 1);

    for &(l, r) in &sections {
        println!("{} {}", l, r);
    }
}

fn dfs(
    graph: &[Vec<usize>],
    sections: &mut [(usize, usize)],
    par: Option<usize>,
    cur: usize,
    pos: &mut usize,
) {
    let start = *pos;
    for &next in &graph[cur] {
        if Some(next) == par {
            continue;
        }

        dfs(graph, sections, Some(cur), next, pos);
        *pos += 1;
    }

    if *pos > start {
        *pos -= 1;
    }
    sections[cur] = (start, *pos);
}
