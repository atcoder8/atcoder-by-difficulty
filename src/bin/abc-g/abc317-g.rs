use ac_library::MfGraph;
use itertools::{iproduct, Itertools};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        mut aaa: [[Usize1; m]; n],
    }

    let mut grid = vec![vec![0; m]; n];
    for col in 0..m {
        let line = create_line(&mut aaa);
        for row in 0..n {
            grid[row][col] = line[row];
        }
    }

    println!("Yes");
    for line in grid {
        println!("{}", line.iter().join(" "));
    }
}

fn create_line(aaa: &mut [Vec<usize>]) -> Vec<usize> {
    let n = aaa.len();

    let start_node = 2 * n;
    let goal_node = start_node + 1;

    let mut mf_graph = MfGraph::<usize>::new(2 * n + 2);

    let mut matchable_num = 0;
    for (val, row) in iproduct!(0..n, 0..n) {
        if aaa[row].contains(&val) {
            mf_graph.add_edge(val, n + row, 1);
            matchable_num += 1;
        }
    }

    for val in 0..n {
        mf_graph.add_edge(start_node, val, 1);
    }

    for row in 0..n {
        mf_graph.add_edge(n + row, goal_node, 1);
    }

    mf_graph.flow(start_node, goal_node);

    let mut line = vec![0; n];
    for edge in mf_graph.edges().into_iter().take(matchable_num) {
        if edge.flow == 1 {
            let (val, row) = (edge.from, edge.to - n);

            line[row] = val + 1;

            let pos = aaa[row].iter().position(|&a| a == val).unwrap();
            aaa[row].remove(pos);
        }
    }

    line
}
