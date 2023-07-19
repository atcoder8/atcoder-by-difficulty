use proconio::{input, marker::Usize1};
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        pp: [Usize1; n - 1],
        q: usize,
        ud: [(Usize1, usize); q],
    }

    let mut children = vec![vec![]; n];
    for (i, &p) in pp.iter().enumerate() {
        children[p].push(i + 1);
    }

    let mut depths = vec![0; n];
    create_depth_vec(&children, &mut depths, 0);

    let mut forward = vec![0; n];
    let mut backward = vec![0; n];
    euler_tour(&children, &mut forward, &mut backward, 0, &mut 0);

    let mut times_per_depth = vec![vec![]; n];
    for i in 0..n {
        times_per_depth[depths[i]].push(forward[i]);
    }
    times_per_depth.iter_mut().for_each(|x| x.sort_unstable());

    for &(u, d) in &ud {
        let times = &times_per_depth[d];
        let ans = times.upper_bound(&backward[u]) - times.lower_bound(&forward[u]);
        println!("{}", ans);
    }
}

fn create_depth_vec(children: &Vec<Vec<usize>>, depths: &mut Vec<usize>, cur: usize) {
    for &child in &children[cur] {
        depths[child] = depths[cur] + 1;
        create_depth_vec(children, depths, child);
    }
}

fn euler_tour(
    children: &Vec<Vec<usize>>,
    forward: &mut Vec<usize>,
    backward: &mut Vec<usize>,
    cur: usize,
    count: &mut usize,
) {
    forward[cur] = *count;
    *count += 1;

    for &child in &children[cur] {
        euler_tour(children, forward, backward, child, count);
    }

    backward[cur] = *count;
    *count += 1;
}
