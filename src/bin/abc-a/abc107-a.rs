use proconio::input;

fn main() {
    input! {
        (n, i): (usize, usize),
    }

    println!("{}", n - i + 1);
}
