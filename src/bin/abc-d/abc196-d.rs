use proconio::input;

fn main() {
    input! {
        (h, w, a, b): (usize, usize, usize, usize),
    }

    let mut ans = 0_usize;

    let mut stack = vec![(vec![vec![false; w]; h], a, b, 0_usize)];

    while let Some((grid, rem_a, rem_b, cell_idx)) = stack.pop() {
        if cell_idx == h * w {
            ans += 1;

            continue;
        }

        let (i, j) = (cell_idx / w, cell_idx % w);

        if grid[i][j] {
            stack.push((grid, rem_a, rem_b, cell_idx + 1));

            continue;
        }

        if rem_a >= 1 {
            if i < h - 1 && !grid[i + 1][j] {
                let mut next_grid = grid.clone();
                next_grid[i][j] = true;
                next_grid[i + 1][j] = true;

                stack.push((next_grid, rem_a - 1, rem_b, cell_idx + 1));
            }

            if j < w - 1 && !grid[i][j + 1] {
                let mut next_grid = grid.clone();
                next_grid[i][j] = true;
                next_grid[i][j + 1] = true;

                stack.push((next_grid, rem_a - 1, rem_b, cell_idx + 1));
            }
        }

        if rem_b >= 1 {
            let mut next_grid = grid.clone();
            next_grid[i][j] = true;

            stack.push((next_grid, rem_a, rem_b - 1, cell_idx + 1));
        }
    }

    println!("{}", ans);
}
