use proconio::input;

fn main() {
    input! {
        (a, b): (usize, usize),
    }

    let ans = 2 * a.max(b) - (a != b) as usize;
    println!("{}", ans);
}
