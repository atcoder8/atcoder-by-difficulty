use proconio::input;

fn main() {
    input! {
        (x, a, b): (i64, i64, i64),
    }

    println!(
        "{}",
        if (x - a).abs() < (x - b).abs() {
            "A"
        } else {
            "B"
        }
    );
}
