use proconio::input;

fn main() {
    input! {
        (_r, g, b): (usize, usize, usize),
    }

    println!("{}", if (10 * g + b) % 4 == 0 { "YES" } else { "NO" });
}
