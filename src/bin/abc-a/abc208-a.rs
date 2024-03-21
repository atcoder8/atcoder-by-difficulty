use proconio::input;

fn main() {
    input! {
        (a, b): (usize, usize),
    }

    let ans = a <= b && b <= 6 * a;
    println!("{}", if ans { "Yes" } else { "No" });
}
