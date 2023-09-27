use proconio::input;

fn main() {
    input! {
        (x, y): (usize, usize),
    }

    println!("{}", if x.abs_diff(y) >= 2 { "Alice" } else { "Brown" });
}
