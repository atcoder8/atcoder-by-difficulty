use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    println!("{}", if n % 3 == 0 { "YES" } else { "NO" });
}
