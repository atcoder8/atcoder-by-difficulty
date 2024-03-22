use proconio::input;

fn main() {
    input! {
        (n, a, x, y): (usize, usize, usize, usize),
    }

    let ans = x * n.min(a) + y * n.saturating_sub(a);
    println!("{}", ans);
}
