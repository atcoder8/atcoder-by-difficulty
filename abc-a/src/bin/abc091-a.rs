use proconio::input;

fn main() {
    input! {
        (a, b, c): (usize, usize, usize),
    }

    println!("{}", if a + b >= c { "Yes" } else { "No" });
}
