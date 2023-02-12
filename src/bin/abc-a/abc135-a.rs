use proconio::input;

fn main() {
    input! {
        (a, b): (usize, usize),
    }

    if a % 2 == b % 2 {
        println!("{}", (a + b) / 2);
    } else {
        println!("IMPOSSIBLE");
    }
}
