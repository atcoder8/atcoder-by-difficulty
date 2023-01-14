use proconio::input;

fn main() {
    input! {
        (n, x): (usize, usize),
    }

    println!("{}", (x - 1).min(n - x));
}
