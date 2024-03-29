use proconio::input;

fn main() {
    input! {
        (mut a, mut b): (usize, usize),
    }

    if a > b {
        std::mem::swap(&mut a, &mut b);
    }

    let ans = (a, b) == (1, 10) || a + 1 == b;
    println!("{}", if ans { "Yes" } else { "No" });
}
