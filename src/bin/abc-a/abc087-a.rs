use proconio::input;

fn main() {
    input! {
        x: usize,
        a: usize,
        b: usize,
    }

    let mut rem = x - a;
    rem -= rem / b * b;

    println!("{}", rem);
}
