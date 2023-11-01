use ac_library::MfGraph;
use itertools::iproduct;
use proconio::input;

const INF: i64 = 10_i64.pow(11);

fn main() {
    input! {
        (h, w): (usize, usize),
        aaa: [[i64; w]; h],
    }

    let start_node = h + w;
    let goal_node = start_node + 1;

    let row_node = |row: usize| row;
    let col_node = |col: usize| h + col;

    let mut mf_graph = MfGraph::new(h + w + 2);

    let set_cost =
        |mf_graph: &mut MfGraph<i64>, node: usize, adopt_cost: i64, not_adopt_cost: i64| {
            mf_graph.add_edge(node, goal_node, adopt_cost);
            mf_graph.add_edge(start_node, node, not_adopt_cost);
        };

    for row in 0..h {
        let penalty = (0..w).map(|col| (-aaa[row][col]).max(0)).sum::<i64>();
        set_cost(&mut mf_graph, row_node(row), penalty, 0);
    }

    for col in 0..w {
        let penalty = (0..h).map(|row| (-aaa[row][col]).max(0)).sum::<i64>();
        set_cost(&mut mf_graph, col_node(col), 0, penalty);
    }

    for (row, col) in iproduct!(0..h, 0..w) {
        if aaa[row][col] < 0 {
            mf_graph.add_edge(row_node(row), col_node(col), INF);
        } else {
            mf_graph.add_edge(col_node(col), row_node(row), aaa[row][col]);
        }
    }

    let mut ans = 0;
    for (row, col) in iproduct!(0..h, 0..w) {
        ans += aaa[row][col].max(0);
    }
    ans -= mf_graph.flow(start_node, goal_node);
    println!("{}", ans);
}
