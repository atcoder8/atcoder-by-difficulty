use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    println!("{}", 800 * n - 200 * (n / 15));
}
