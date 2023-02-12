use proconio::input;

fn main() {
    input! {
        (mut a, mut d): (usize, usize),
    }

    if a <= d {
        a += 1;
    } else {
        d += 1;
    }

    println!("{}", a * d);
}
