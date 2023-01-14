use proconio::input;

fn main() {
    input! {
        (x, t): (usize, usize),
    }

    println!("{}", x - t.min(x));
}
