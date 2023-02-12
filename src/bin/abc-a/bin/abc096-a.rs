use proconio::input;

fn main() {
    input! {
        (a, b): (usize, usize),
    }

    let ans = a - 1 + (a <= b) as usize;
    println!("{}", ans);
}
