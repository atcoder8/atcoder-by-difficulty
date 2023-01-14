use proconio::input;

fn main() {
    input! {
        (a, b, c, d): (i64, i64, i64, i64),
    }

    let ans = (a - c).abs() <= d || ((a - b).abs() <= d && (b - c).abs() <= d);
    println!("{}", if ans { "Yes" } else { "No" });
}
