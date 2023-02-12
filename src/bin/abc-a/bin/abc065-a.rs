use proconio::input;

fn main() {
    input! {
        (x, a, b): (usize, usize, usize),
    }

    let ans = if a >= b {
        "delicious"
    } else if b - a <= x {
        "safe"
    } else {
        "dangerous"
    };
    println!("{}", ans);
}
