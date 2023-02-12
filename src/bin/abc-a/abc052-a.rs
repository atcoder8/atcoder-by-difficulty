use proconio::input;

fn main() {
    input! {
        (a, b, c, d): (usize, usize, usize, usize),
    }

    println!("{}", (a * b).max(c * d));
}
