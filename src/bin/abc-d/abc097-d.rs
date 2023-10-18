use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        pp: [Usize1; n],
        xy: [(Usize1, Usize1); m],
    }

    let mut graph = vec![vec![]; n];
    for &(x, y) in &xy {
        graph[x].push(y);
        graph[y].push(x);
    }

    let mut ans = 0;
    let mut used = vec![false; n];
    for start in 0..n {
        let mut stack = vec![start];
        let mut indexes = vec![];
        let mut values = vec![];

        while let Some(cur) = stack.pop() {
            if used[cur] {
                continue;
            }

            used[cur] = true;

            indexes.push(cur);
            values.push(pp[cur]);

            for &next in &graph[cur] {
                stack.push(next);
            }
        }

        indexes.sort_unstable();
        values.sort_unstable();

        let mut idx1 = 0;
        let mut idx2 = 0;
        while idx1 < indexes.len() && idx2 < values.len() {
            if indexes[idx1] < values[idx2] {
                idx1 += 1;
            } else if indexes[idx1] > values[idx2] {
                idx2 += 1;
            } else {
                ans += 1;
                idx1 += 1;
                idx2 += 1;
            }
        }
    }

    println!("{}", ans);
}
