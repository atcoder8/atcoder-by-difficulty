use proconio::input;

fn main() {
    input! {
        (a, b, t): (usize, usize, usize),
    }

    println!("{}", t / a * b);
}
