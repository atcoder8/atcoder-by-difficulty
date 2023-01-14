use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    println!("{}", if n / 100 == n % 10 { "Yes" } else { "No" });
}
