use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [i64; n],
    }

    let mut bb = vec![0; n];
    for i in 0..(n - 1) {
        bb[i + 1] = bb[i] + aa[i / 2];
    }

    let mut dp = vec![vec![-2e18 as i64; n]; n];
    dp[0][0] = 0;
    for i in 0..(n - 1) {
        for j in 0..(n - 1) {
            chmax!(dp[i + 1][j + 1], dp[i][j]);
            chmax!(dp[i + 1][0], dp[i][j] + bb[j]);
        }
    }

    let ans: i64 = (0..n).map(|i| dp[n - 1][i] + bb[i]).max().unwrap();
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
