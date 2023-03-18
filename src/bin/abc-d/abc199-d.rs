use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        ab: [(Usize1, Usize1); m],
    }

    let mut graph = vec![vec![]; n];
    for &(a, b) in &ab {
        graph[a].push(b);
        graph[b].push(a);
    }
    graph.iter_mut().for_each(|x| x.sort_unstable());

    let mut ans = 1;
    let mut colors = vec![None; n];
    let mut visited = vec![false; n];

    for start_node in 0..n {
        if visited[start_node] {
            continue;
        }

        colors[start_node] = Some(0);

        ans *= 3 * dfs(&mut colors, start_node, &graph);

        colors[start_node] = None;

        add_visitable_nodes(&graph, &colors, start_node, &mut visited);
    }

    println!("{}", ans);
}

fn add_visitable_nodes(
    graph: &Vec<Vec<usize>>,
    colors: &Vec<Option<usize>>,
    node: usize,
    visitable: &mut Vec<bool>,
) {
    visitable[node] = true;

    for &next_node in &graph[node] {
        if colors[next_node].is_none() && !visitable[next_node] {
            add_visitable_nodes(graph, colors, next_node, visitable);
        }
    }
}

fn dfs(colors: &mut Vec<Option<usize>>, node: usize, graph: &Vec<Vec<usize>>) -> usize {
    let n = colors.len();

    if graph[node]
        .iter()
        .any(|&next_node| colors[next_node] == colors[node])
    {
        return 0;
    }

    if graph[node]
        .iter()
        .all(|&next_node| colors[next_node].is_some())
    {
        return 1;
    }

    let mut comb = 1;
    let mut visited = vec![false; n];

    for &next_node in &graph[node] {
        if colors[next_node].is_some() || visited[next_node] {
            continue;
        }

        let mut sub_comb = 0;

        for next_color in 0..3 {
            if Some(next_color) == colors[node] {
                continue;
            }

            colors[next_node] = Some(next_color);

            sub_comb += dfs(colors, next_node, graph);

            colors[next_node] = None;
        }

        comb *= sub_comb;

        add_visitable_nodes(graph, colors, next_node, &mut visited);
    }

    comb
}
