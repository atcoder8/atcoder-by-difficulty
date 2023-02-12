use proconio::input;

fn main() {
    input! {
        x: usize,
    }

    let ans = x == 3 || x == 5 || x == 7;
    println!("{}", if ans { "YES" } else { "NO" });
}
