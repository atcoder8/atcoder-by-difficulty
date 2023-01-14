use proconio::input;

fn main() {
    input! {
        coords: [usize; 5],
        k: usize,
    }

    println!(
        "{}",
        if coords[4] - coords[0] > k {
            ":("
        } else {
            "Yay!"
        }
    );
}
