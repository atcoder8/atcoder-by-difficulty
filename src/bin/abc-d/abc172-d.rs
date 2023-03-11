use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let ans: usize = (1..=n).map(|k| k * (n / k) * (n / k + 1) / 2).sum();
    println!("{}", ans);
}
