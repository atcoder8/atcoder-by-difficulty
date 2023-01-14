use proconio::input;

fn main() {
    input! {
        a: usize,
    }

    let half_a = a / 2;

    println!("{}", half_a * (a - half_a));
}
