use proconio::input;

fn main() {
    input! {
        x: usize,
    }

    println!("{}", if x < 1200 { "ABC" } else { "ARC" });
}
