use proconio::input;

fn main() {
    input! {
        (a, b, c): (usize, usize, usize),
    }

    let ans = 2 * (a * b + b * c + c * a);
    println!("{}", ans);
}
