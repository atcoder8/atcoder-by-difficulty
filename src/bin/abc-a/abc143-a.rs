use proconio::input;

fn main() {
    input! {
        (a, b): (usize, usize),
    }

    println!("{}", a.saturating_sub(2 * b));
}
