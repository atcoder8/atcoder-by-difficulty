use proconio::input;

fn main() {
    input! {
        (a, b): (i64, i64),
    }

    println!("{}", (a + b).max(a - b).max(a * b));
}
