use proconio::input;

fn main() {
    input! {
        (a, b): (usize, usize),
    }

    let ans = a * b - (a + b - 1);
    println!("{}", ans);
}
