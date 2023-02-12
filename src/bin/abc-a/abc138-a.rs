use proconio::input;

fn main() {
    input! {
        a: usize,
        s: String,
    }

    println!("{}", if a >= 3200 { &s } else { "red" });
}
