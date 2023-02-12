use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let ans = if n % 2 == 0 { n } else { 2 * n };
    println!("{}", ans);
}
