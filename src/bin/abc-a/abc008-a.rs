use proconio::input;

fn main() {
    input! {
        (s, t): (usize, usize),
    }

    println!("{}", t - s + 1);
}
