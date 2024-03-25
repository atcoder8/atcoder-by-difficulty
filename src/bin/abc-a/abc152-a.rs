use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
    }

    println!("{}", if n == m { "Yes" } else { "No" });
}
