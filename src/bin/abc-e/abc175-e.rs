use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (r, c, k): (usize, usize, usize),
        rcv: [(Usize1, Usize1, usize); k],
    }

    let mut grid = vec![vec![0; c]; r];
    for &(r, c, v) in &rcv {
        grid[r][c] = v;
    }

    let mut dp = vec![vec![vec![0; 4]; c]; r];
    dp[0][0][1] = grid[0][0];
    for row in 0..r {
        for col in 0..c {
            for i in 0..=3 {
                if row > 0 {
                    chmax!(dp[row][col][0], dp[row - 1][col][i]);
                    chmax!(dp[row][col][1], dp[row - 1][col][i] + grid[row][col]);
                }

                if col > 0 {
                    chmax!(dp[row][col][i], dp[row][col - 1][i]);

                    if i != 0 {
                        chmax!(dp[row][col][i], dp[row][col - 1][i - 1] + grid[row][col]);
                    }
                }
            }
        }
    }

    let ans = *dp[r - 1][c - 1].iter().max().unwrap();
    println!("{}", ans);
}

/// If the right-hand side is greater than the left-hand side,
/// clones the right-hand side, bind it to the left-hand side,
/// and return `true`.
/// If the right-hand side is less than or equal to the left-hand side,
/// does nothing and returns `false`.
///
/// The left-hand and right-hand sides must be the same type
/// and must implement the `Clone` trait.
///
/// # Examples
///
/// ```
/// let mut x = 5;
///
/// assert_eq!(chmax!(x, 3), false);
/// assert_eq!(x, 5);
///
/// assert_eq!(chmax!(x, 7), true);
/// assert_eq!(x, 7);
/// ```
#[macro_export]
macro_rules! chmax {
    ($lhs: expr, $rhs: expr) => {
        if $rhs > $lhs {
            let temp = $rhs.clone();
            $lhs = temp;
            true
        } else {
            false
        }
    };
}
