use im_rc::{HashMap, HashSet};
use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    }

    let mut graph = HashMap::<usize, HashSet<usize>>::new();
    for &(u, v) in &ab {
        graph.entry(u).or_default().insert(v);
        graph.entry(v).or_default().insert(u);
    }

    let mut visited = HashSet::<usize>::new();
    let mut stack = vec![1];
    while let Some(cur) = stack.pop() {
        if visited.contains(&cur) {
            continue;
        }

        visited.insert(cur);

        if let Some(edges) = graph.get(&cur) {
            stack.extend(edges.iter().cloned());
        }
    }

    let ans = *visited.iter().max().unwrap();
    println!("{}", ans);
}
