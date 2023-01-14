use proconio::input;

fn main() {
    input! {
        (a, b, c): (usize, usize, usize),
    }

    let ans = c.saturating_sub(a - b);
    println!("{}", ans);
}
