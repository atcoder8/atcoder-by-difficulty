use proconio::input;

fn main() {
    input! {
        (a, b): (usize, usize),
    }

    let ans = if a >= 13 {
        b
    } else if a >= 6 {
        b / 2
    } else {
        0
    };
    println!("{}", ans);
}
