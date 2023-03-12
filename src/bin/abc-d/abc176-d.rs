use std::collections::VecDeque;

use proconio::{
    input,
    marker::{Chars, Usize1},
};

const DIFFS: [(usize, usize); 4] = [(!0, 0), (0, !0), (0, 1), (1, 0)];

fn main() {
    if let Some(ans) = solve() {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}

fn solve() -> Option<usize> {
    input! {
        (h, w): (usize, usize),
        (ch, cw): (Usize1, Usize1),
        (dh, dw): (Usize1, Usize1),
        ss: [Chars; h],
    }

    let mut warp_diffs = vec![];
    for i in 0_usize..5 {
        for j in 0_usize..5 {
            warp_diffs.push((i.wrapping_sub(2), j.wrapping_sub(2)));
        }
    }

    let mut dist_mat = vec![vec![None; w]; h];
    dist_mat[ch][cw] = Some(0);

    let mut queue = VecDeque::from(vec![(ch, cw)]);

    while !queue.is_empty() {
        let mut visited: Vec<_> = queue.iter().cloned().collect();

        while let Some((cur_x, cur_y)) = queue.pop_front() {
            for &(diff_x, diff_y) in &DIFFS {
                let next_x = cur_x.wrapping_add(diff_x);
                let next_y = cur_y.wrapping_add(diff_y);

                if next_x >= h || next_y >= w || ss[next_x][next_y] == '#' {
                    continue;
                }

                if dist_mat[next_x][next_y].is_none() {
                    dist_mat[next_x][next_y] = dist_mat[cur_x][cur_y];
                    visited.push((next_x, next_y));
                    queue.push_back((next_x, next_y));
                }
            }
        }

        while let Some((cur_x, cur_y)) = visited.pop() {
            for &(diff_x, diff_y) in &warp_diffs {
                let next_x = cur_x.wrapping_add(diff_x);
                let next_y = cur_y.wrapping_add(diff_y);

                if next_x >= h || next_y >= w || ss[next_x][next_y] == '#' {
                    continue;
                }

                if ss[next_x][next_y] == '.' && dist_mat[next_x][next_y].is_none() {
                    dist_mat[next_x][next_y] = Some(dist_mat[cur_x][cur_y].unwrap() + 1);
                    queue.push_back((next_x, next_y));
                }
            }
        }
    }

    return dist_mat[dh][dw];
}
