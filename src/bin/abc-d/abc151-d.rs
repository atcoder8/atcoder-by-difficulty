use std::collections::VecDeque;

use proconio::{input, marker::Chars};

const DIFFS: [(usize, usize); 4] = [(!0, 0), (0, !0), (0, 1), (1, 0)];

fn main() {
    input! {
        (h, w): (usize, usize),
        sss: [Chars; h],
    }

    let find_max_dist = |(start_x, start_y): (usize, usize)| -> Option<usize> {
        if sss[start_x][start_y] == '#' {
            return None;
        }

        let mut visited = vec![vec![false; w]; h];
        visited[start_x][start_y] = true;

        let mut queue = VecDeque::from(vec![((start_x, start_y), 0)]);
        queue.push_back(((start_x, start_y), 0_usize));

        let mut max_dist = 0;

        while let Some(((cur_x, cur_y), cur_dist)) = queue.pop_front() {
            for &(diff_x, diff_y) in &DIFFS {
                let next_x = cur_x.wrapping_add(diff_x);
                let next_y = cur_y.wrapping_add(diff_y);
                let next_dist = cur_dist + 1;

                if next_x < h
                    && next_y < w
                    && sss[next_x][next_y] == '.'
                    && !visited[next_x][next_y]
                {
                    visited[next_x][next_y] = true;
                    max_dist = max_dist.max(next_dist);
                    queue.push_back(((next_x, next_y), next_dist));
                }
            }
        }

        Some(max_dist)
    };

    let ans: usize = (0..h)
        .flat_map(|start_x| (0..w).map(move |start_y| find_max_dist((start_x, start_y))))
        .filter_map(|max_dist| max_dist)
        .max()
        .unwrap();
    println!("{}", ans);
}
