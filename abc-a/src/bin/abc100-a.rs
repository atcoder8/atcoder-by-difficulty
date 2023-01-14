use proconio::input;

fn main() {
    input! {
        (a, b): (usize, usize),
    }

    println!("{}", if a <= 8 && b <= 8 { "Yay!" } else { ":(" });
}
