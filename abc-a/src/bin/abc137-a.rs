use proconio::input;

fn main() {
    input! {
        (a, b): (i64, i64),
    }

    let ans = (a + b).max(a - b).max(a * b);
    println!("{}", ans);
}
