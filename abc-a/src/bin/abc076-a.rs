use proconio::input;

fn main() {
    input! {
        (r, g): (i64, i64),
    }

    println!("{}", 2 * g - r);
}
