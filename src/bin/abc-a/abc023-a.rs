use proconio::input;

fn main() {
    input! {
        x: usize,
    }

    println!("{}", x / 10 + x % 10);
}
