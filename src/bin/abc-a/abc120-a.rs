use proconio::input;

fn main() {
    input! {
        (a, b, c): (usize, usize, usize),
    }

    let ans = (b / a).min(c);
    println!("{}", ans);
}
