use proconio::input;

fn main() {
    input! {
        (x, y): (usize, usize),
    }

    println!("{}", y / x);
}
