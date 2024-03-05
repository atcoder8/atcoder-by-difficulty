use proconio::input;

fn main() {
    input! {
        (a, b, c, d): (usize, usize, usize, usize),
    }

    println!(
        "{}",
        if a < c || a == c && b <= d {
            "Takahashi"
        } else {
            "Aoki"
        }
    );
}
