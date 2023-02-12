use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
    }

    let ans = if n % k == 0 { 0 } else { 1 };
    println!("{}", ans);
}
