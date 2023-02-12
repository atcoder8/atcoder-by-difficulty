use proconio::input;

fn main() {
    input! {
        (a, p): (usize, usize),
    }

    println!("{}", (3 * a + p) / 2);
}
