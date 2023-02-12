use proconio::input;

fn main() {
    input! {
        q: usize,
    }

    println!("{}", if q == 1 { "ABC" } else { "chokudai" });
}
