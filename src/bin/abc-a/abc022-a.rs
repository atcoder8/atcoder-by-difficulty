use proconio::input;

fn main() {
    input! {
        (n, s, t): (usize, i64, i64),
        mut w: i64,
        aa: [i64; n - 1],
    }

    let is_best = |x| s <= x && x <= t;

    let mut ans = is_best(w) as i64;
    for &a in &aa {
        w += a;
        ans += is_best(w) as i64;
    }

    println!("{}", ans);
}
