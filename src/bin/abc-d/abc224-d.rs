use std::collections::VecDeque;

use im_rc::HashSet;
use itertools::Itertools;
use proconio::{input, marker::Usize1};

const N: usize = 9;

fn main() {
    if let Some(ans) = solve() {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}

fn solve() -> Option<usize> {
    input! {
        m: usize,
        uv: [(Usize1, Usize1); m],
        pp: [Usize1; N - 1],
    }

    let mut graph = vec![vec![]; N];
    for &(a, b) in &uv {
        graph[a].push(b);
        graph[b].push(a);
    }
    graph.iter_mut().for_each(|x| x.sort_unstable());

    let complete = (0..(N - 1)).collect_vec();

    let mut used = HashSet::new();
    used.insert(pp.clone());
    let mut queue = VecDeque::new();
    queue.push_back((pp, 0));

    while let Some((puzzle, cnt)) = queue.pop_front() {
        if puzzle == complete {
            return Some(cnt);
        }

        let empty_node = (0..N).find(|&i| !puzzle.contains(&i)).unwrap();

        for &next_empty_node in &graph[empty_node] {
            let mut next_puzzle = puzzle.clone();
            let move_node = puzzle
                .iter()
                .find_position(|&&node| node == next_empty_node)
                .unwrap()
                .0;
            next_puzzle[move_node] = empty_node;

            if !used.contains(&next_puzzle) {
                used.insert(next_puzzle.clone());
                queue.push_back((next_puzzle, cnt + 1));
            }
        }
    }

    None
}
