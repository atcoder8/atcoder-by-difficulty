use proconio::input;

fn main() {
    input! {
        n: usize,
        (x, y): (usize, usize),
        ab: [(usize, usize); n],
    }

    let mut dp: Vec<Vec<Option<usize>>> = vec![vec![None; y + 1]; x + 1];
    dp[0][0] = Some(0);

    for &(a, b) in &ab {
        for i in (0..=x).rev() {
            for j in (0..=y).rev() {
                if let Some(cnt) = dp[i][j] {
                    let next = &mut dp[(i + a).min(x)][(j + b).min(y)];
                    if next.is_none() || cnt + 1 < next.unwrap() {
                        *next = Some(cnt + 1);
                    }
                }
            }
        }
    }

    if let Some(ans) = dp[x][y] {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}
