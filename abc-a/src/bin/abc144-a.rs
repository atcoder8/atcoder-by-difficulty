use proconio::input;

fn main() {
    input! {
        (a, b): (usize, usize),
    }

    if a <= 9 && b <= 9 {
        println!("{}", a * b);
    } else {
        println!("-1");
    }
}
