use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    println!("{}", if n <= 999 { "ABC" } else { "ABD" });
}
