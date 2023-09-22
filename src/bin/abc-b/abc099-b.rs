use proconio::input;

fn main() {
    input! {
        (a, b): (usize, usize),
    }

    let ans = (b - a - 1) * (b - a) / 2 - a;
    println!("{}", ans);
}
