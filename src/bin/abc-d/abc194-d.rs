use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let ans: f64 = (1..=(n - 1)).map(|k| n as f64 / k as f64).sum();
    println!("{}", ans);
}
