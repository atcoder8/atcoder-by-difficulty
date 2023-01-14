use proconio::input;

fn main() {
    input! {
        (n, a, b): (usize, usize, usize),
    }

    println!("{}", (a * n).min(b));
}
