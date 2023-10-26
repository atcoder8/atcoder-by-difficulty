use itertools::Itertools;
use proconio::{input, marker::Usize1};

const DIFFS: [(usize, usize); 4] = [(!0, 0), (0, !0), (0, 1), (1, 0)];

fn main() {
    input! {
        n: usize,
        pp: [Usize1; n.pow(2)],
    }

    let mut depths = vec![vec![0; n + 2]; n + 2];
    for (i, j) in (0..n).cartesian_product(0..n) {
        depths[i + 1][j + 1] = i.min(j).min(n - 1 - i).min(n - 1 - j) + 1;
    }

    let mut ans = 0;
    let mut leaved = vec![vec![false; n + 2]; n + 2];
    for &p in &pp {
        let (row, col) = (p / n + 1, p % n + 1);

        ans += depths[row][col] - 1;
        leaved[row][col] = true;

        let min_depth = DIFFS
            .iter()
            .map(|&(diff_row, diff_col)| {
                let nei_row = row.wrapping_add(diff_row);
                let nei_col = col.wrapping_add(diff_col);

                depths[nei_row][nei_col]
            })
            .min()
            .unwrap();

        let mut stack = vec![(row, col, min_depth)];
        while let Some((row, col, depth)) = stack.pop() {
            if depth >= depths[row][col] {
                continue;
            }

            depths[row][col] = depth;

            for (diff_row, diff_col) in DIFFS {
                let next_row = row.wrapping_add(diff_row);
                let next_col = col.wrapping_add(diff_col);
                let next_depth = depth + !leaved[next_row][next_col] as usize;

                stack.push((next_row, next_col, next_depth));
            }
        }
    }

    println!("{}", ans);
}
