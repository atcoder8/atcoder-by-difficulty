use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let odd_num = (n + 1) / 2;

    println!("{}", odd_num as f64 / n as f64);
}
