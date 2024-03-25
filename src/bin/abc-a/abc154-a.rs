use proconio::input;

fn main() {
    input! {
        (s, _t): (String, String),
        (mut a, mut b): (usize, usize),
        u: String,
    }

    if u == s {
        a -= 1;
    } else {
        b -= 1;
    }

    println!("{} {}", a, b);
}
