use proconio::input;

fn main() {
    input! {
        x: usize,
    }

    println!(
        "{}",
        if x >= 100 && x % 100 == 0 {
            "Yes"
        } else {
            "No"
        }
    );
}
