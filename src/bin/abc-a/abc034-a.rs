use proconio::input;

fn main() {
    input! {
        (x, y): (usize, usize),
    }

    println!("{}", if x < y { "Better" } else { "Worse" });
}
